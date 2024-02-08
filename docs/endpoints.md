# Endpoints on `api`-Service

The following endpoints with base URL `api/` are implemented in
[`handlers.rs`](../components/api/handlers.rs):

_TODO: This section could be auto generated._

## `PUT` **`/api/v1/{user_id}/keys/{key_id}`**:

- **Description**: Stores the public and encrypted private keys under user id
  `user_id`. The key id must not exist already.

- **Method**: `PUT`
- **Parameters**:

  - `user_id`: The user id [`str`]. **Should probably be `uuid` later too for
    better security**.
  - `key_id` : The key id [`uuid`].

- **Request Body**: [`json`]

  ```json
  {
    "version": "1",
    "public_key": "...",
    "private_key_encrypted": "..."
  }
  ```

- Responses:

  - Status `200` if the key pair was created.
  - Status `403` if the key id `key_id` already exists.

## `DELETE` **`/api/v1/{user_id}/keys/{key_id}`**:

- **Method**: `DELETE`
- **Parameters**:

  - `user_id`: The user id [`str`]. **Should probably be `uuid` later too for
    better security**.
  - `key_id` : The key id [`uuid`].

- Responses:

  - Status `200` if the key pair was deleted (may have not existed).

## `GET` **`/api/v1/{user_id}/keys/{key_id}`**:

- **Description**: Gets the public and encrypted private key for key id `key_id`
  of user `user_id`. If it does not exists.

- **Method**: `GET`
- **Parameters**:

  - `user_id`: The user id [`str`]. **Should probably be `uuid` later too for
    better security**.
  - `key_id` : The key id [`uuid`].

- **Response Body**: [`json`]

  ```json
  {
    "version": "1",
    "public_key": "...",
    "private_key_encrypted": "..."
  }
  ```

- Responses:
  - Status `200` if the key pair exists and was returned.
  - Status `404` if the key pair for `key_id` was not found, due to missing user
    id or key id.
