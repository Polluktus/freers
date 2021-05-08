fn normal(contains: &String, search_pattern: String) -> i32 {

    let mut mem_lines: Vec<String> = Vec::new();
    for i in contains.lines() {
       if i.contains(&search_pattern) {
           mem_lines.push(i.to_string());
       }
    } 

    let mut values: Vec<i32> = Vec::new();
    for i in mem_lines[0].split_whitespace() {
        let val = match i.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        values.push(val);
    }
    values[0]
}

pub fn basic(contains: &String) {
    let mem_total = normal(&contains, String::from("MemTotal"));
    let mem_free = normal(&contains, String::from("MemFree"));
    let buffers = normal(&contains, String::from("Buffers"));
    let sreclaimable = normal(&contains, String::from("SReclaimable"));
    let cached = normal(&contains, String::from("Cached"));
    let shmem = normal(&contains, String::from("Shmem"));
    let mem_available = normal(&contains, String::from("MemAvailable"));

    let swap_total = normal(&contains, String::from("SwapTotal"));
    let swap_free = normal(&contains, String::from("SwapFree"));
    let swap_cached = normal(&contains, String::from("SwapCached"));
    
    let buf_cached = buffers + sreclaimable + cached;
    let swap_used = swap_total - swap_free - swap_cached;
    let mem_used = mem_total - mem_free - buffers - cached;
    let tmp = format!("
                razem       użyte       wolne       dzielone       buf/cache       dostępne
Pamięć:      {}     {}    {}         {}         {}       {} 
Wymiana:      {}           {}     {} "
    ,mem_total, mem_used, mem_free , shmem, buf_cached, mem_available, swap_total, swap_used, swap_free);
    println!("{}", tmp);
}
pub fn human(contains: &String) {
    let mem_total = normal(&contains, String::from("MemTotal"));
    let mem_free = normal(&contains, String::from("MemFree"));
    let buffers = normal(&contains, String::from("Buffers"));
    let sreclaimable = normal(&contains, String::from("SReclaimable"));
    let cached = normal(&contains, String::from("Cached"));
    let shmem = normal(&contains, String::from("Shmem"));
    let mem_available = normal(&contains, String::from("MemAvailable"));

    let swap_total = normal(&contains, String::from("SwapTotal"));
    let swap_free = normal(&contains, String::from("SwapFree"));
    let swap_cached = normal(&contains, String::from("SwapCached"));
    
    let buf_cached = buffers + sreclaimable + cached;
    let swap_used = swap_total - swap_free - swap_cached;
    let mem_used = mem_total - mem_free - buffers - cached;
    let tmp = format!("
                razem       użyte       wolne       dzielone       buf/cache       dostępne
Pamięć:         {}Mi    {}Mi     {}Mi          {}Mi           {}Mi        {}Mi 
Wymiana:         {}Mi       {}Mi      {}Mi"
    ,mem_total/1024, mem_used/1024, mem_free/1024, shmem/1024, buf_cached/1024, mem_available/1024, swap_total/1024, swap_used/1024, swap_free/1024);
    println!("{}", tmp);
}