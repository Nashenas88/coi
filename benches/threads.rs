#![allow(soft_unstable)]
#![feature(test)]
extern crate test;

use coi::{coi, container};
use std::sync::Arc;
use test::Bencher;


trait I {}
#[coi(provides dyn I + Send + Sync with S)]
struct S;

impl I for S {}

#[coi(provides MySql with MySql)]
struct MySql;

#[coi(provides MongoDb with MongoDb)]
struct MongoDb;

trait UserRepository {}

#[coi(provides dyn UserRepository + Send + Sync with UserRepo::new(mysql_db))]
struct UserRepo {
    #[coi(inject)]
    mysql_db: Arc<MySql>,
}

impl UserRepo {
    pub fn new(mysql_db: Arc<MySql>) -> Self {
        Self { mysql_db }
    }
}

impl UserRepository for UserRepo {}

trait BillingRepository {}

#[coi(provides dyn BillingRepository + Send + Sync with BillingRepo::new(mongo_db))]
struct BillingRepo {
    #[coi(inject)]
    mongo_db: Arc<MongoDb>,
}

impl BillingRepo {
    pub fn new(mongo_db: Arc<MongoDb>) -> Self {
        Self { mongo_db }
    }
}

impl BillingRepository for BillingRepo {}

trait ShoppingCartRepository {}

#[coi(provides dyn ShoppingCartRepository + Send + Sync with
    ShoppingCartRepo::new(mongo_db)
)]
struct ShoppingCartRepo {
    #[coi(inject)]
    mongo_db: Arc<MongoDb>,
}

impl ShoppingCartRepo {
    pub fn new(mongo_db: Arc<MongoDb>) -> Self {
        Self { mongo_db }
    }
}

impl ShoppingCartRepository for ShoppingCartRepo {}

trait ShoppingCartService {}

#[coi(provides dyn ShoppingCartService + Send + Sync with
    ShoppingCartSvc::new(shopping_cart_repo, user_repo)
)]
struct ShoppingCartSvc {
    #[coi(inject)]
    shopping_cart_repo: Arc<dyn ShoppingCartRepository + Send + Sync>,
    #[coi(inject)]
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl ShoppingCartSvc {
    pub fn new(
        shopping_cart_repo: Arc<dyn ShoppingCartRepository + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        Self {
            shopping_cart_repo,
            user_repo,
        }
    }
}

impl ShoppingCartService for ShoppingCartSvc {}

trait AccountService {}

#[coi(provides dyn AccountService + Send + Sync with
    AccountSvc::new(user_repo)
)]
struct AccountSvc {
    #[coi(inject)]
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl AccountSvc {
    pub fn new(user_repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repo }
    }
}

impl AccountService for AccountSvc {}

trait LoginService {}

#[coi(provides dyn LoginService + Send + Sync with
    LoginSvc::new(user_repo)
)]
struct LoginSvc {
    #[coi(inject)]
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl LoginSvc {
    pub fn new(user_repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repo }
    }
}

impl LoginService for LoginSvc {}

trait PaymentProvider {}

#[coi(provides dyn PaymentProvider + Send + Sync with Strayp)]
struct Strayp;

impl PaymentProvider for Strayp {}

trait PaymentService {}

#[coi(provides dyn PaymentService + Send + Sync with
    PaymentSvc::new(payment_provider, user_repo, shopping_cart_repo, billing_repo)
)]
struct PaymentSvc {
    #[coi(inject)]
    payment_provider: Arc<dyn PaymentProvider + Send + Sync>,
    #[coi(inject)]
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    #[coi(inject)]
    shopping_cart_repo: Arc<dyn ShoppingCartRepository + Send + Sync>,
    #[coi(inject)]
    billing_repo: Arc<dyn BillingRepository + Send + Sync>,
}

impl PaymentSvc {
    pub fn new(
        payment_provider: Arc<dyn PaymentProvider + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
        shopping_cart_repo: Arc<dyn ShoppingCartRepository + Send + Sync>,
        billing_repo: Arc<dyn BillingRepository + Send + Sync>,
    ) -> Self {
        Self {
            payment_provider,
            user_repo,
            shopping_cart_repo,
            billing_repo,
        }
    }
}

impl PaymentService for PaymentSvc {}

#[coi(provides Guid with Guid)]
struct Guid;

#[bench]
fn run_15_background_threads_while_resolving(b: &mut Bencher) {
    use std::sync::atomic::AtomicBool;

    let container = Arc::new(container! {
        guid => GuidProvider,
        mysql_db => MySqlProvider; scoped,
        mongo_db => MongoDbProvider; scoped,
        user_repo => UserRepoProvider; scoped,
        billing_repo => BillingRepoProvider; scoped,
        shopping_cart_repo => ShoppingCartRepoProvider; scoped,
        shopping_cart_svc => ShoppingCartSvcProvider; scoped,
        account_svc => AccountSvcProvider; scoped,
        login_svc => LoginSvcProvider; scoped,
        payment_provider => StraypProvider; singleton,
        payment_svc => PaymentSvcProvider; scoped,
    });

    let done = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::with_capacity(15);

    for _ in 0..5 {
        let done = done.clone();
        let container = container.clone();
        let handle = std::thread::spawn(move || {
            loop {
                let container = container.scoped();
                container.resolve::<Guid>("guid").unwrap();
                container.resolve::<dyn LoginService + Send + Sync>("login_svc").unwrap();
                if done.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for _ in 0..5 {
        let done = done.clone();
        let container = container.clone();
        let handle = std::thread::spawn(move || {
            loop {
                let container = container.scoped();
                container.resolve::<Guid>("guid").unwrap();
                container.resolve::<dyn ShoppingCartService + Send + Sync>("shopping_cart_svc").unwrap();
                if done.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for _ in 0..5 {
        let done = done.clone();
        let container = container.clone();
        let handle = std::thread::spawn(move || {
            loop {
                let container = container.scoped();
                container.resolve::<Guid>("guid").unwrap();
                container.resolve::<dyn AccountService + Send + Sync>("account_svc").unwrap();
                if done.load(std::sync::atomic::Ordering::Acquire) {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    b.iter(|| {
        let container = container.scoped();
        container.resolve::<Guid>("guid").unwrap();
        container.resolve::<dyn PaymentService + Send + Sync>("payment_svc").unwrap();
    });

    done.store(true, std::sync::atomic::Ordering::Release);

    for handle in handles {
        handle.join().unwrap();
    }
}