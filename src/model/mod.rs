// region:    --- Modules

mod error;
mod store;
pub mod task;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use self::store::{new_db_pool, Db};

pub use self::error::{Error, Result};

/*
 Model Layer

Design:

	- The Model layer normalizes the application's data type structures and access.
	- All application code data access must go through the Model Layer.
	- The `ModaleManager` holds the internal state/resources needed by ModelControllers to access data.
	  (e.g., db_pool, S3 client, redis client, etc.)
	- Model Controllers (e.g., `TaskBMC`, `ProjectBMC`, etc.) implement CRUD and other data access methods on given "entity".
	  (e.g., Task, project, etc.)
	  (BMC is short for Backend Model Controller)
	- In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
	- ModelManager are designed to be passed as argument to all model controllers functions.
*/

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;

		// FIXME - TBC
		Ok(ModelManager { db })
	}

	// Returns the sqlx db pool reference only for model layer internal use.
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
