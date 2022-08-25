pub trait UserEntityImp{
    pub fn is_email_valid(email:&str)->bool;
    pub fn is_name_valid(name:&str)->bool;
    pub fn new(user:UserEntity) -> Result<UserEntity,&'static str>;
}