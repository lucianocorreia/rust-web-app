#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:5001")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	//logoff
	// let req_logoff = hc.do_post("/api/logoff", json!({"logoff": true}));
	// req_logoff.await?.print().await?;

	// hc.do_get("/hello").await?.print().await?;

	// - RPC - create task
	let req_rpc_create = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_task",
			"params": {
				"data": {
					"title": "Task 1",
				}
			}
		}),
	);
	req_rpc_create.await?.print().await?;

	// - RPC - update task
	let req_rpc_update = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": 1000,
				"data": {
					"title": "Task 11111",
				}
			}
		}),
	);
	req_rpc_update.await?.print().await?;

	// -- RPC - list tasks
	let req_list_tasks =
		hc.do_post("/api/rpc", json!({"id": 1, "method": "list_tasks"}));
	req_list_tasks.await?.print().await?;

	// - RPC - delete task
	let req_rpc_delete = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": 900,
			}
		}),
	);
	req_rpc_delete.await?.print().await?;

	Ok(())
}
