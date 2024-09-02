#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct User {
    id: u64,
    name: String,
    email: String,
    created_at: u64,
}

impl User {
    fn new(id: u64, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Will {
    id: u64,
    user_id: u64,
    executor_id: u64,
    assets: Vec<Asset>,
    beneficiaries: Vec<Beneficiary>,
    created_at: u64,
    is_executed: bool,
}

impl Will {
    fn new(id: u64, user_id: u64, executor_id: u64) -> Self {
        Self {
            id,
            user_id,
            executor_id,
            assets: Vec::new(),
            beneficiaries: Vec::new(),
            created_at: time(),
            is_executed: false,
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Asset {
    id: u64,
    will_id: u64,
    name: String,
    value: u64,
    created_at: u64,
}

impl Asset {
    fn new(id: u64, will_id: u64, name: String, value: u64) -> Self {
        Self {
            id,
            will_id,
            name,
            value,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Beneficiary {
    id: u64,
    will_id: u64,
    name: String,
    share: u8,
    created_at: u64,
}

impl Beneficiary {
    fn new(id: u64, will_id: u64, name: String, share: u8) -> Self {
        Self {
            id,
            will_id,
            name,
            share,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Executor {
    id: u64,
    name: String,
    contact: String,
    created_at: u64,
}

impl Executor {
    fn new(id: u64, name: String, contact: String) -> Self {
        Self {
            id,
            name,
            contact,
            created_at: time(),
        }
    }
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Will {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Will {
    const MAX_SIZE: u32 = 4096;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Asset {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Asset {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Beneficiary {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Beneficiary {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Executor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Executor {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static WILLS_STORAGE: RefCell<StableBTreeMap<u64, Will, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ASSETS_STORAGE: RefCell<StableBTreeMap<u64, Asset, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static BENEFICIARIES_STORAGE: RefCell<StableBTreeMap<u64, Beneficiary, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static EXECUTORS_STORAGE: RefCell<StableBTreeMap<u64, Executor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

// Payloads Definitions

// Payloads for creating and managing users
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    name: String,
    email: String,
}

// Payloads for creating and managing wills
#[derive(candid::CandidType, Deserialize, Serialize)]
struct WillPayload {
    user_id: u64,
    executor_id: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct AssetPayload {
    will_id: u64,
    name: String,
    value: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct BeneficiaryPayload {
    will_id: u64,
    name: String,
    share: u8,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ExecutorPayload {
    name: String,
    contact: String,
}

// AsignExecutor Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AssignExecutorPayload {
    will_id: u64,
    executor_id: u64,
}

// Function to create and manage users

#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, String> {
    // Ensure the user payload is valid

    if payload.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if payload.email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    // Generate a new ID for the user
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let user = User::new(id, payload.name, payload.email);
    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(user.id, user.clone()));
    Ok(user)
}

// Functions to create executor
#[ic_cdk::update]
fn create_executor(payload: ExecutorPayload) -> Result<Executor, String> {
    // Ensure the executor payload is valid
    if payload.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if payload.contact.is_empty() {
        return Err("Contact cannot be empty".to_string());
    }

    // Generate a new ID for the executor
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let executor = Executor::new(id, payload.name, payload.contact);

    EXECUTORS_STORAGE.with(|storage| storage.borrow_mut().insert(executor.id, executor.clone()));
    Ok(executor)
}

// Functions to create and manage entities

#[ic_cdk::update]
fn create_will(payload: WillPayload) -> Result<Will, String> {
    // Ensure the user id and executor id are valid
    if USERS_STORAGE
        .with(|storage| storage.borrow().get(&payload.user_id))
        .is_none()
    {
        return Err("User not found".to_string());
    }

    if EXECUTORS_STORAGE
        .with(|storage| storage.borrow().get(&payload.executor_id))
        .is_none()
    {
        return Err("Executor not found".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let will = Will::new(id, payload.user_id, payload.executor_id);
    WILLS_STORAGE.with(|storage| storage.borrow_mut().insert(will.id, will.clone()));
    Ok(will)
}

#[ic_cdk::update]
fn add_asset(payload: AssetPayload) -> Result<Asset, String> {
    // Validate the asset payload
    if payload.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Validate the asset value
    if payload.value == 0 {
        return Err("Value cannot be zero".to_string());
    }

    // Ensure the will id is valid
    if WILLS_STORAGE
        .with(|storage| storage.borrow().get(&payload.will_id))
        .is_none()
    {
        return Err("Will not found".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let asset = Asset::new(id, payload.will_id, payload.name, payload.value);
    ASSETS_STORAGE.with(|storage| storage.borrow_mut().insert(asset.id, asset.clone()));
    Ok(asset)
}

#[ic_cdk::update]
fn add_beneficiary(payload: BeneficiaryPayload) -> Result<Beneficiary, String> {
    // Validate the beneficiary payload
    if payload.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Validate the beneficiary share
    if payload.share == 0 {
        return Err("Share cannot be zero".to_string());
    }

    // Ensure the will id is valid
    let mut will = WILLS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&payload.will_id)
            .clone()
            .ok_or_else(|| "Will not found".to_string())
    })?;

    // Generate a new ID for the beneficiary
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let beneficiary = Beneficiary::new(id, payload.will_id, payload.name, payload.share);

    // Add the new beneficiary to the will
    will.beneficiaries.push(beneficiary.clone());

    // Update the will in storage with the new beneficiary added
    WILLS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(will.id, will.clone());
    });

    // Store the beneficiary in the beneficiaries storage
    BENEFICIARIES_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(beneficiary.id, beneficiary.clone())
    });

    Ok(beneficiary)
}

#[ic_cdk::update]
fn assign_executor(payload: AssignExecutorPayload) -> Result<Will, String> {
    WILLS_STORAGE.with(|storage| {
        let mut storage_ref = storage.borrow_mut();

        // Retrieve the will, return an error if not found
        let mut will = storage_ref
            .get(&payload.will_id)
            .ok_or_else(|| "Will not found".to_string())?
            .clone();

        // Ensure the executor id is valid
        if EXECUTORS_STORAGE
            .with(|storage| storage.borrow().get(&payload.executor_id))
            .is_none()
        {
            return Err("Executor not found".to_string());
        }

        // Modify the executor_id
        will.executor_id = payload.executor_id;

        // Insert the modified will back into the storage
        storage_ref.insert(payload.will_id, will.clone());

        // Return the updated will
        Ok(will)
    })
}

// Retrieve functions

#[ic_cdk::query]
fn get_user(user_id: u64) -> Result<User, String> {
    USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&user_id)
            .clone()
            .ok_or_else(|| "User not found".to_string())
    })
}

#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, String> {
    USERS_STORAGE.with(|storage| {
        let users: Vec<User> = storage
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();
        if users.is_empty() {
            Err("No users found.".to_string())
        } else {
            Ok(users)
        }
    })
}

#[ic_cdk::query]
fn get_executor(executor_id: u64) -> Result<Executor, String> {
    EXECUTORS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&executor_id)
            .clone()
            .ok_or_else(|| "Executor not found".to_string())
    })
}

#[ic_cdk::query]
fn get_all_executors() -> Result<Vec<Executor>, String> {
    EXECUTORS_STORAGE.with(|storage| {
        let executors: Vec<Executor> = storage
            .borrow()
            .iter()
            .map(|(_, executor)| executor.clone())
            .collect();
        if executors.is_empty() {
            Err("No executors found.".to_string())
        } else {
            Ok(executors)
        }
    })
}

#[ic_cdk::query]
fn get_will(will_id: u64) -> Result<Will, String> {
    WILLS_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&will_id)
            .clone()
            .ok_or_else(|| "Will not found".to_string())
    })
}

#[ic_cdk::query]
fn get_all_wills() -> Result<Vec<Will>, String> {
    WILLS_STORAGE.with(|storage| {
        let wills: Vec<Will> = storage
            .borrow()
            .iter()
            .map(|(_, will)| will.clone())
            .collect();
        if wills.is_empty() {
            Err("No wills found.".to_string())
        } else {
            Ok(wills)
        }
    })
}

#[ic_cdk::query]
fn get_all_assets() -> Result<Vec<Asset>, String> {
    ASSETS_STORAGE.with(|storage| {
        let assets: Vec<Asset> = storage
            .borrow()
            .iter()
            .map(|(_, asset)| asset.clone())
            .collect();
        if assets.is_empty() {
            Err("No assets found.".to_string())
        } else {
            Ok(assets)
        }
    })
}

#[ic_cdk::query]
fn get_all_beneficiaries() -> Result<Vec<Beneficiary>, String> {
    BENEFICIARIES_STORAGE.with(|storage| {
        let beneficiaries: Vec<Beneficiary> = storage
            .borrow()
            .iter()
            .map(|(_, beneficiary)| beneficiary.clone())
            .collect();
        if beneficiaries.is_empty() {
            Err("No beneficiaries found.".to_string())
        } else {
            Ok(beneficiaries)
        }
    })
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// Candid export
ic_cdk::export_candid!();
