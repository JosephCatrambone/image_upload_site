use perseus::prelude::*;
use perseus::plugins::Plugins;
use serde::{Serialize, Deserialize};
use sycamore::prelude::*;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "UserStateRx")]
struct UserState {
	user_id: Uuid, // let id = Uuid::new_v4() or uuid!("string literal uuid");
	//#[rx(nested)]  // Another thing.
}

enum User {
	AuthenticatedUser(AuthenticatedUser),
	
}

// This function will be run when you build your app, to generate default state ahead-of-time.
// The whole user should be conditional, not just UUID.
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> UserState {
    UserState {
        user_id: Uuid::new_v4(),
    }
}