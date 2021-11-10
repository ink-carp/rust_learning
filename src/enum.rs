enum Ip
{
    Ip4:(u8,u8,u8,u8),
    Ip6:String,
}
fn main()
{
    let myip = Ip::Ip4(127,0,0,1);
}
