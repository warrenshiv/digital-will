#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
// use chrono::{Utc, DateTime};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Customer {
    id: u64,
    name: String,
    contact: String,
    email: String,
    created_at: u64,
}

impl Customer {
    fn new(id: u64, name: String, contact: String, email: String) -> Self {
        Self {
            id,
            name,
            contact,
            email,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Vehicle {
    id: u64,
    customer_id: u64,
    make: String,
    model: String,
    year: u32,
    license_plate: String,
    created_at: u64,
}

impl Vehicle {
    fn new(
        id: u64,
        customer_id: u64,
        make: String,
        model: String,
        year: u32,
        license_plate: String,
    ) -> Self {
        Self {
            id,
            customer_id,
            make,
            model,
            year,
            license_plate,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Service {
    id: u64,
    vehicle_id: u64,
    description: String,
    cost: u32,
    date: u64,
    created_at: u64,
}

impl Service {
    fn new(id: u64, vehicle_id: u64, description: String, cost: u32, date: u64) -> Self {
        Self {
            id,
            vehicle_id,
            description,
            cost,
            date,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Inventory {
    id: u64,
    part_name: String,
    quantity: u32,
    cost: u32,
    created_at: u64,
}

impl Inventory {
    fn new(id: u64, part_name: String, quantity: u32, cost: u32) -> Self {
        Self {
            id,
            part_name,
            quantity,
            cost,
            created_at: time(),
        }
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
struct Invoice {
    id: u64,
    customer_id: u64,
    amount: u32,
    date: u64,
    created_at: u64,
}

impl Invoice {
    fn new(id: u64, customer_id: u64, amount: u32, date: u64) -> Self {
        Self {
            id,
            customer_id,
            amount,
            date,
            created_at: time(),
        }
    }
}

impl Storable for Customer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Customer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Vehicle {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Vehicle {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Service {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Service {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Inventory {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Inventory {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Invoice {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Invoice {
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

    static CUSTOMERS_STORAGE: RefCell<StableBTreeMap<u64, Customer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static VEHICLES_STORAGE: RefCell<StableBTreeMap<u64, Vehicle, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static SERVICES_STORAGE: RefCell<StableBTreeMap<u64, Service, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static INVENTORY_STORAGE: RefCell<StableBTreeMap<u64, Inventory, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static INVOICES_STORAGE: RefCell<StableBTreeMap<u64, Invoice, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}

// Payloads Definitions

// Customer Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct CustomerPayload {
    name: String,
    contact: String,
    email: String,
}

// Vehicle Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct VehiclePayload {
    customer_id: u64,
    make: String,
    model: String,
    year: u32,
    license_plate: String,
}

// Service Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct ServicePayload {
    vehicle_id: u64,
    description: String,
    cost: u32,
}

// Inventory Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct InventoryPayload {
    part_name: String,
    quantity: u32,
    cost: u32,
}

//  Invoice Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct InvoicePayload {
    customer_id: u64,
    amount: u32,
}

// Functions to create and get entities

#[ic_cdk::update]
fn create_customer(payload: CustomerPayload) -> Result<Customer, String> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err("Name, contact, and email cannot be empty".to_string());
    }

    // Validate email format
    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err("Invalid email format".to_string());
    }

    // Validate contact format
    let contact_regex = Regex::new(r"^\d{10}$").unwrap();
    if !contact_regex.is_match(&payload.contact) {
        return Err("Invalid contact format".to_string());
    }

    // Validate email uniqueness
    let email_exists = CUSTOMERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, customer)| customer.email == payload.email)
    });

    if email_exists {
        return Err("Email already exists".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let customer = Customer::new(id, payload.name, payload.contact, payload.email);
    CUSTOMERS_STORAGE.with(|storage| storage.borrow_mut().insert(customer.id, customer.clone()));
    Ok(customer)
}

// Function to update the profile of the user
#[ic_cdk::update]
fn update_customer(id: u64, payload: CustomerPayload) -> Result<Customer, String> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err("Name, contact, and email cannot be empty".to_string());
    }

    // Validate the customer exists
    let customer_exists = CUSTOMERS_STORAGE.with(|storage| storage.borrow().contains_key(&id));
    if !customer_exists {
        return Err("Customer ID does not exist.".to_string());
    }

    // Validate email format
    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err("Invalid email format".to_string());
    }

    // Validate contact format
    let contact_regex = Regex::new(r"^\d{10}$").unwrap();
    if !contact_regex.is_match(&payload.contact) {
        return Err("Invalid contact format".to_string());
    }

    // Validate email uniqueness
    let email_exists = CUSTOMERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, customer)| customer.email == payload.email)
    });

    if email_exists {
        return Err("Email already exists".to_string());
    }

    let customer = Customer::new(id, payload.name, payload.contact, payload.email);
    CUSTOMERS_STORAGE.with(|storage| storage.borrow_mut().insert(customer.id, customer.clone()));
    Ok(customer)
}

// Function to retrieve all customers and throw an error if no customers are found
#[ic_cdk::query]
fn get_all_customers() -> Result<Vec<Customer>, String> {
    CUSTOMERS_STORAGE.with(|storage| {
        let customers: Vec<Customer> = storage
            .borrow()
            .iter()
            .map(|(_, customer)| customer.clone())
            .collect();
        if customers.is_empty() {
            Err("No customers found.".to_string())
        } else {
            Ok(customers)
        }
    })
}

#[ic_cdk::update]
fn create_vehicle(payload: VehiclePayload) -> Result<Vehicle, String> {
    if payload.make.is_empty() || payload.model.is_empty() || payload.license_plate.is_empty() {
        return Err("Make, model, and license plate cannot be empty".to_string());
    }

    // Check if customer exists
    let customer_exists =
        CUSTOMERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.customer_id));
    if !customer_exists {
        return Err("Customer ID does not exist.".to_string());
    }

    // Validate license plate format
    let license_plate_regex = Regex::new(r"^[A-Z]{2}-\d{2}-[A-Z]{2}-\d{4}$").unwrap();
    if !license_plate_regex.is_match(&payload.license_plate) {
        return Err("Invalid license plate format".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let vehicle = Vehicle::new(
        id,
        payload.customer_id,
        payload.make,
        payload.model,
        payload.year,
        payload.license_plate,
    );
    VEHICLES_STORAGE.with(|storage| storage.borrow_mut().insert(vehicle.id, vehicle.clone()));
    Ok(vehicle)
}

// Function to retrieve all vehicles, handling the case where no vehicles are found
#[ic_cdk::query]
fn get_all_vehicles() -> Result<Vec<Vehicle>, String> {
    VEHICLES_STORAGE.with(|storage| {
        let vehicles: Vec<Vehicle> = storage
            .borrow()
            .iter()
            .map(|(_, vehicle)| vehicle.clone())
            .collect();
        if vehicles.is_empty() {
            Err("No vehicles found.".to_string())
        } else {
            Ok(vehicles)
        }
    })
}

#[ic_cdk::update]
fn create_service(payload: ServicePayload) -> Result<Service, String> {
    if payload.description.is_empty() {
        return Err("Description cannot be empty".to_string());
    }

    // Check if vehicle exists
    let vehicle_exists =
        VEHICLES_STORAGE.with(|storage| storage.borrow().contains_key(&payload.vehicle_id));
    if !vehicle_exists {
        return Err("Vehicle ID does not exist.".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let service = Service::new(
        id,
        payload.vehicle_id,
        payload.description,
        payload.cost,
        time(),
    );
    SERVICES_STORAGE.with(|storage| storage.borrow_mut().insert(service.id, service.clone()));
    Ok(service)
}

// Function to retrieve all services, handling the case where no services are found
#[ic_cdk::query]
fn get_all_services() -> Result<Vec<Service>, String> {
    SERVICES_STORAGE.with(|storage| {
        let services: Vec<Service> = storage
            .borrow()
            .iter()
            .map(|(_, service)| service.clone())
            .collect();
        if services.is_empty() {
            Err("No services found.".to_string())
        } else {
            Ok(services)
        }
    })
}

#[ic_cdk::update]
fn create_inventory_item(payload: InventoryPayload) -> Result<Inventory, String> {
    if payload.part_name.is_empty() {
        return Err("Part name cannot be empty".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let inventory = Inventory::new(id, payload.part_name, payload.quantity, payload.cost);
    INVENTORY_STORAGE.with(|storage| storage.borrow_mut().insert(inventory.id, inventory.clone()));
    Ok(inventory)
}

// Function to retrieve the quantity of a specific part name in the inventory
#[ic_cdk::query]
fn get_inventory_quantity(part_name: String) -> Result<u32, String> {
    INVENTORY_STORAGE.with(|storage| {
        let items: Vec<Inventory> = storage
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect();

        let item = items.iter().find(|item| item.part_name == part_name);

        match item {
            Some(inventory_item) => Ok(inventory_item.quantity),
            None => Err("Part name not found in inventory.".to_string()),
        }
    })
}

// Function to retrieve all inventory items, handling the case where no items are found
#[ic_cdk::query]
fn get_all_inventory_items() -> Result<Vec<Inventory>, String> {
    INVENTORY_STORAGE.with(|storage| {
        let items: Vec<Inventory> = storage
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect();
        if items.is_empty() {
            Err("No inventory items found.".to_string())
        } else {
            Ok(items)
        }
    })
}

#[ic_cdk::update]
fn create_invoice(payload: InvoicePayload) -> Result<Invoice, String> {
    // Check if customer exists
    let customer_exists =
        CUSTOMERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.customer_id));
    if !customer_exists {
        return Err("Customer ID does not exist.".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let invoice = Invoice::new(id, payload.customer_id, payload.amount, time());
    INVOICES_STORAGE.with(|storage| storage.borrow_mut().insert(invoice.id, invoice.clone()));
    Ok(invoice)
}

// Function to get customer invoices using customer id and sum the invoices
#[ic_cdk::query]
fn get_customer_invoices(customer_id: u64) -> Result<u32, String> {
    let invoices = INVOICES_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, invoice)| invoice.customer_id == customer_id)
            .map(|(_, invoice)| invoice.amount)
            .collect::<Vec<u32>>()
    });

    if invoices.is_empty() {
        return Err("No invoices found for the customer.".to_string());
    }

    Ok(invoices.iter().sum())
}

// Function to retrieve all invoices, handling the case where no invoices are found
#[ic_cdk::query]
fn get_all_invoices() -> Result<Vec<Invoice>, String> {
    INVOICES_STORAGE.with(|storage| {
        let invoices: Vec<Invoice> = storage
            .borrow()
            .iter()
            .map(|(_, invoice)| invoice.clone())
            .collect();
        if invoices.is_empty() {
            Err("No invoices found.".to_string())
        } else {
            Ok(invoices)
        }
    })
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
