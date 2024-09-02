# Garage Management System

This project is a decentralized platform built on the Internet Computer for managing customers, vehicles, services, inventory, and invoices in a garage. It allows users to create and manage records for customers, their vehicles, services performed on those vehicles, inventory items, and invoices.

## Key Features

1. **Customer Management**

   - **Add Customer:** Allows users to create customer profiles.
   - **Update Customer:** Allows users to update existing customer profiles.
   - **Get All Customers:** Retrieve a list of all customers in the system.

2. **Vehicle Management**

   - **Add Vehicle:** Allows users to add vehicles to the system.
   - `license_plate: AB-12-CD-3456`
   - **Get All Vehicles:** Retrieve a list of all vehicles in the system.

3. **Service Management**

   - **Create Service:** Allows users to record services performed on vehicles.
   - **Get All Services:** Retrieve a list of all services performed.

4. **Inventory Management**

   - **Create Inventory Item:** Allows users to add items to the inventory.
   - **Get Inventory Quantity:** Retrieve the quantity of a specific part in the inventory.
   - **Get All Inventory Items:** Retrieve a list of all items in the inventory.

5. **Invoice Management**
   - **Create Invoice:** Allows users to create invoices for customers.
   - **Get Customer Invoices:** Retrieve and sum invoices for a specific customer.
   - **Get All Invoices:** Retrieve a list of all invoices.

## Error Handling

- **Not Found:** Returns an error if a requested item is not found.
- **Unauthorized Access:** Returns an error if a user tries to perform an action without necessary permissions.

# Sample Payloads

## Sample Payloads

### CustomerPayload

```json
{
  "name": "John Doe",
  "contact": "0734566787",
  "email": "john.doe@example.com"
}
```

### VehiclePayload

```json
{
  "customer_id": 1,
  "make": "Toyota",
  "model": "Corolla",
  "year": 2020,
  "license_plate": "AB-12-CD-3456"
}
```

### ServicePayload

```json
{
  "vehicle_id": 1,
  "description": "Oil change",
  "cost": 50
}
```

### InventoryPayload

```json
{
  "part_name": "Brake Pad",
  "quantity": 100,
  "cost": 25
}
```

### InvoicePayload

```json
{
  "customer_id": 1,
  "amount": 150
}
```

## Requirements

- rustc 1.64 or higher

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```

- rust wasm32-unknown-unknown target

```bash
$ rustup target add wasm32-unknown-unknown
```

- candid-extractor

```bash
$ cargo install candid-extractor
```

- install `dfx`

```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:

```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
