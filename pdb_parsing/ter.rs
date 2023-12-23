fn main() {
    let ter_line = "TER    1187      LEU B  75A";
    let res_seq: i32 = ter_line[22..26].trim().parse().unwrap();
    let icode = if ter_line.len() > 26 { 
	ter_line[26..27].chars().next().unwrap() 
    } else {' '};
    let chain_id: &str = ter_line[21..22].trim();
    
    println!("chain name: {}, residue number: {}", chain_id, res_seq);
}