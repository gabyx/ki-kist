# Architecture

The following describes the architecture of the components `api` and the
involved other parts it interacts with. Not implemented parts in all diagrams
are marked with: ⚒ .

## Component Diagram

<div style="text-align:center">
    <img src="deployment.drawio.svg" alt="Architecture"/>
</div>

Since the backend handles a very vulnerable part (maybe part of a larger
application), namely storing asymmetric keys, it is crucial to employ the
strictest security measures possible. The following interactions happen:

1. The `client` communicates with the backend over `HTTP/2.0` requests which are
   TLS encrypted (version >=`1.2`).

   - The client should only store the keys it interacts with in physical memory
     and only decrypt the private key when really needed. We treat the CLI
     application here as a client which might have the same runtime life-time as
     a corresponding web application.

   - The client does not log in anyway the key pairs it interacts with.

   - In case of the CLI:
     - it handles exit codes properly (an absolutely horrifiying example would
       be JFrogs Artifactory CLI `jf`).
     - it tries to either prompt the user to enter the encryption token for the
       private key generation or it reads it from a file (argument
       `--encryption-token <file>`) to avoid leaking anything to the process
       listing (e.g. `ps`).

2. The entry point in the backend consists of a `firewall` [⚒] which handles:

   - threat prevention (e.g.against denial-of-service (DDoS) attacks)
   - access control (VPN etc. (?))
   - application layer filtering etc.

3. A `reverse proxy` [⚒] (e.g. `nginx`) receives the requests and handles user
   authentication such that the `api`-service can focus on its relevant tasks
   and does not need to to do authentication again.

4. The `api`-service is scaled by multiple instances (depending on the load
   etc.) and handles these [endpoints](#endpoints-on-api-service).

   - The `api` is dumb and only validates the keys it receives and then stores
     them into the `database` (e.g. `PostgreSQL`) under the user id and given
     key id. The user id is of type UUID Version 4 which increases security. The
     same holds for the key id.

   - The private keys are already encrypted and the `api` has no knowledge of
     the encryption token given to the `client` to possibly decrypt them.

5. The database should generally only allow encrypted connections over TLS. This
   means to configure the [`postgresdb`](../manifests/postgresdb) to use TLS
   with certificates [⚒].

## Endpoints on `api`-Service

The following endpoints with base URL `api/` are implemented in
[`handlers.rs`](../components/api/handlers.rs):

_TODO: This section could be auto generated._

- **`api/v1/user/{user_id}/store/{key_id}`**:

  - **Description**: Stores the public and encrypted private keys under user id
    `user_id`. If the `key_id` does not exist it will be created.

  - **Method**: `PUT`.
  - **Parameters**:

    - `user_id`: The user id [`uuid`].
    - `key_id` : The key id [`uuid`].

  - **Request Body**: [`json`]

    ```json
    "version" = "1"
    "public_key" = "..."
    "private_key_encrypted" = "..."
    ```

  - Responses:
    - Status `200` if the key pair was overwritten.
    - Status `201` if the key pair was created.

- **`api/v1/user/{user_id}/store/{key_id}`**:

  - **Description**: Gets the public and encrypted private key for key id
    `key_id` for the user `user_id`. If it does not exists.

  - **Method**: `GET`.
  - **Parameters**:

    - `user_id`: The user id [`uuid`].
    - `key_id` : The key id [`uuid`].

  - **Response Body**: [`json`]

    ```json
    "version" = "1"
    "public_key" = "..."
    "private_key_encrypted" = "..."
    ```

  - Responses:
    - Status `200` if the key pair exists and was returned.
    - Status `403` if the user does not exist.
    - Status `404` if the key pair for `key_id` was not found.

## Database Tables

The data base persists jobs in the `jobs` table and job logs (received on the
`status`-queue) in the `job-logs` table.

```mermaid
erDiagram
    jobs {
        uuid id PK "Job id."

        enum status "Job status: 'pending', 'queued', 'done'."
        string blob_digest "Blob SHA265 digest."
        string name "The name of the job."
        timestamp creation_time "The time the job was created."
        timestamp end_time "The time the job was finished."
        enum converter_result "The converter result ('failure', 'success')"
        enum converter_log "The converter full log (stdout/stderr)"
    }

    job-logs {
        uuid id PK "The log id."
        uuid job FK "The job id this message belongs to."

        timestamp timestamp "The log's timestamp."
        text message "The status message."
        enum severity "The severity of the status message ('normal', 'error')."
    }
```
