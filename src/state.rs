use anchor_client::{
	Program,
	Client,
	Cluster,
	EventContext,
	solana_sdk::{
		pubkey::Pubkey,
		commitment_config::CommitmentConfig,
		signature::{
			Keypair,
			Signer,
			read_keypair_file,
		}
	},
};
use std::{
	fs,
	rc::Rc,
	sync::Arc,
	str::FromStr,
};
use arc_swap::ArcSwap;

pub struct DnState{
	pub mango_client: ArcSwap<Arc<Program>>,
	pub payer: dyn Signer,
	// idk add other stuff in here too XD
}

// What crack did you smoke to be writing this wtf...
fn read_keys(path: &str) -> Vec<Arc<Pubkey>> {
	return fs::read_to_string(path)
		.unwrap()
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| Arc::new(Pubkey::from_str(s).unwrap()))
		.collect();
}

// meh this will probably get bigger
impl DnState{
	pub fn new() -> &'static DnState {
		// Config Settings
		let url = Cluster::Mainnet;
		let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/payer.json"))
			.expect("Wya wyd keypair???");

		let client = 
			Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

		let dn_state = DnState {
			mango_program: ArcSwap::from_pointee(Arc::new(client.program())),
		};

		Box::leak(Box::new(dn_state))
	}
}