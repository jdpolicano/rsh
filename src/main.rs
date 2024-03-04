use rsh::shell::Rsh;

fn main() {
   let mut rsh = Rsh::new(">>> ".to_string());
   rsh.run().unwrap();
}
