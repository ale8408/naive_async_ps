use std::net::Ipv4Addr;
use clap::Parser;

use crate::types_manager::*;

#[derive(Parser, Debug)]
#[command(about = "First async attempt. Scan ports given range(s).", long_about = None)]
struct ArgumentsBeforeParse {
	target_ip: String,
	#[arg(short, long)]
	port_ranges: Option<String>,
}

pub fn args_to_vars() -> (Ipv4Addr, Vec<PortRange>) {
	let abp = ArgumentsBeforeParse::parse();

	let target_ip: Ipv4Addr = abp.target_ip.parse().unwrap();
	let port_ranges: Vec<PortRange> = match abp.port_ranges {
		Some(range) => parse_port_ranges(range),
		None => vec![PortRange::new(1u16, 65535u16)],
	};

	(target_ip, port_ranges)
}

fn parse_port_ranges(port_input: String) -> Vec<PortRange> {
	port_input
		.split(",")
		.map(|range| {
			match range.find("-") {
				Some(position) => PortRange::new(
									(&range[0..position])
										.to_string()
										.parse()
										.unwrap(),
									(&range[position+1..])
										.to_string()
										.parse()
										.unwrap(),
									),
				None => PortRange::new(
							range
								.to_string()
								.parse()
								.unwrap(),
							range
								.to_string()
								.parse()
								.unwrap()
							),
			}
		})
		.collect()
}