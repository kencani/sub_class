pub fn total(param : &[u32] ) -> Option<u32>{
    let mut total_temp  = 0u32;
    for i in param{
        if u32::MAX - i >= total_temp{
            total_temp = total_temp + i;
        }else{
            return None;
        }
    }
    Some(total_temp)
}