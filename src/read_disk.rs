use std::fs;


#[derive(Debug)]
pub struct DiskInfo {
    disk_name:String,
    disk_kb:u128
}

fn get_info_from(s:&str) -> Option<DiskInfo> {
    //udev /dev devtmpfs rw,nosuid,relatime,size=7902152k,nr_inodes=1975538,mode=755,inode64 0 0
    let mut attrs = s.split(" ");
    attrs.next();
    let mount_point = attrs.next().expect("Mount point could not be read");
    attrs.next();
    let mut size_str = attrs.next().expect("Size attribute could not be read").split("=");
    size_str.next();
    let size = size_str.next().expect("Size attribute could not be read").strip_suffix("k").expect("Size attribute does not end in K for some reason");
    let info = DiskInfo {
        disk_name:mount_point.to_owned(),
        disk_kb:size.parse().unwrap(),
    };
    Some(info)
}

fn has_size_attr(s:&str) -> bool {
    return s.contains("size=");
}

///reads the /proc/mounts file to provide disk information
pub fn read_disk_of_system() ->Vec<DiskInfo> {
    let proc_mounts_file_contents = fs::read_to_string("/proc/mounts")
        .expect("/proc/mounts could not be read, if you are on windows this will not work. If on linux the problem might have something to do with privileges");
    let lines_read = proc_mounts_file_contents.split("\n");
    lines_read.filter(|l| !l.is_empty()).filter(|l| has_size_attr(l)).map(|l| get_info_from(l)).flatten().collect()
}
