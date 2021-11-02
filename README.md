# GNAP Authorization Service in Rust

This is a learning sanbox to explore building full stack services in Rust, while
maybe actually doing someting useful.

This service is developed with Actix, Serde, Redis, and MongoDB.

## Workspace Structure

This crate leverages the Cargo workspace concept to isolate various architectural
components into their own Rust libs.
The libs are listed in the top level [Cargo.toml](./Cargo.toml) file.

### [error](./error)
First attempt to consolidate error management, so that all Results return a
[GnapError](./error/src/lib.rs). This should be extended for any new errors.

### [dao](./dao)
The data persistence is managed via MongoDB.  The [dao](./dao) lib provides an
abstraction level between the REST handlers and the database.  The dao lib
defines a [Service](./dao/src/lib.rs) that encapsulates the [GnapDB](./dao/src/db.rs)
and the [GnapCache](./dao/src/cache.rs).

### [model](./model)
All model structs, for DAO persistence, and HTTP requests/responses.  This lib
relies heavily on serde (serde_json) to serialize/deserialize.  The lib also
leverages Redis to manage appropriate (de)serialization for the cache.

### [as](./as)
The GNAP Authorization Server.  This service is built with Actix.
The handlers understand the data models as defined in the [model](./model) lib.

## Starting Mongo and Redis
This service leverages MongoDB and Redis.  Both are run in Docker containers

### Launch MongoDB and Redis

To launch mongo and redis via docker, run:

````
> docker-compose up -d
````

MongoDB and Redis containers will be started, and linked to local folders as a
volumes.  Review the [docker-compose.yml](./docker-compose.yml) file for info.

The MongoDB instance is initialized with the script in [mongodb-init](./mongodb-init/init.js).
You can extend this script to initialize with more data.  Note: this script is
only used once.  So, you will need to delete the top level `data` folder and
relaunch the containers in order to take effect.

Access the monodb shell with:

````bash
> docker exec -it mongodb mongo
````

### Manage the Env Settings
Copy the following in a `.env` file in the workspace root (where the top level Cargo.toml lives):

````
MONGODB_URI=mongodb://127.0.0.1:27017
MONGODB_DATABASE=gnap
MONGODB_USER=me
MONGODB_PASSWORD=password
MONGODB_APP_NAME=gnap
REDIS_URI=redis://localhost
API_ADDRESS=0.0.0.0:8000
RUST_LOG=actix_todo=debug,actix_web=info,r#as=trace
````

## Run

- Start Mongo and Redis containers:

````bash
> docker-compose up -d
````

- Start the server
from the top level workspace folder, run the following:

````bash
> cargo run
````

The service will run on 0.0.0.0:8000.  You can change this by updating the [.env](./.env) file.

## Interacting with the Service
There is a Postman collection in the root folder.  Import that.


## Extending the Service

### Step 1: Update the Model
The service leverages `serde_json` for all request and response data.  Updating
and extending the model starts with defining a `struct` for each request body
and each response in [model](./model).  If an API accepts data via query params (which I strongly
suggest avoiding - use post!!), the query params can be modeled via a `struct`
as well.

### Step 2: Add the Routes
The routes are defined in [as/src/routes](./as/src/routes).
You can bundle the routes in
separate modules by adding additional files in this folder.

### Step 3: Add the Handlers
The handlers are defined in [as/src/handlers](./as/src/handlers).
Each handler sbould be defined to
accept the `service` parameter (for access to the db client), and whatever body,
path params, and query params managed by the route.

### Step 4:  Add the route to the routes method
At the end of each route module, there is a `routes` function.  This function is
called in [main](./as/src/main.rs) to construct the Actix routes.  Just
follow the pattern to add your new route.

