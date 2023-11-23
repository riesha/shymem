#![feature(core_intrinsics)]
use ntapi::{
    ntmmapi::{NtCreateSection, NtMapViewOfSection, ViewUnmap},
    ntpsapi::NtCurrentProcess,
    nttmapi::NtCreateTransaction,
};
use ntstatus::ntstatus::NtStatus;
use std::{
    fs::File,
    io::prelude::*,
    mem::{self, size_of},
    os::windows::io::FromRawHandle,
    ptr, thread, time,
};
use winapi::{
    self,
    shared::ntdef::OBJECT_ATTRIBUTES,
    um::winnt::{
        FILE_ATTRIBUTE_NORMAL, GENERIC_READ, GENERIC_WRITE, PAGE_READWRITE, SECTION_ALL_ACCESS,
        SEC_COMMIT,
    },
};
use windows::{
    core::PCSTR,
    Win32::Storage::FileSystem::{CreateFileTransactedA, CREATE_NEW, FILE_SHARE_MODE},
};
type HANDLE = *mut ntapi::winapi::ctypes::c_void;
fn main()
{
    let mut handle: HANDLE = ptr::null_mut();
    let mut object_attr: OBJECT_ATTRIBUTES = unsafe { mem::zeroed() };
    object_attr.Length = size_of::<OBJECT_ATTRIBUTES>() as _;

    let tx_status = NtStatus::from_u32(unsafe {
        NtCreateTransaction(
            ptr::addr_of_mut!(handle),
            0x12003F,
            ptr::addr_of_mut!(object_attr),
            ptr::null_mut() as _,
            ptr::null_mut() as _,
            0,
            0,
            0,
            ptr::null_mut() as _,
            ptr::null_mut() as _,
        ) as u32
    });

    dbg!(tx_status);

    if tx_status == Some("STATUS_SUCCESS")
    {
        let tx_handle = unsafe {
            CreateFileTransactedA(
                PCSTR::from_raw(b"edsjgfsdgsdg\0".as_ptr()),
                GENERIC_WRITE | GENERIC_READ,
                FILE_SHARE_MODE(0),
                None,
                CREATE_NEW,
                windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(
                    FILE_ATTRIBUTE_NORMAL,
                ),
                None,
                windows::Win32::Foundation::HANDLE(handle as _),
                None,
                None,
            )
            .unwrap()
        };

        dbg!(tx_handle);

        let mut file = unsafe { File::from_raw_handle(tx_handle.0 as _) };
        file.write_all(b"123").unwrap();
        let mut section: HANDLE = ptr::null_mut();

        dbg!(NtStatus::from_u32(unsafe {
            NtCreateSection(
                ptr::addr_of_mut!(section),
                SECTION_ALL_ACCESS,
                ptr::null_mut(),
                ptr::null_mut(),
                PAGE_READWRITE,
                SEC_COMMIT,
                tx_handle.0 as _,
            ) as u32
        }));

        let mut base: HANDLE = ptr::null_mut();

        for i in (39..=46).rev()
        {
            let mut view_size: usize = (1u64 << i) as _;

            let mapview_status = NtStatus::from_u32(unsafe {
                NtMapViewOfSection(
                    section,
                    NtCurrentProcess,
                    ptr::addr_of_mut!(base),
                    0,
                    0x1000,
                    ptr::null_mut(),
                    ptr::addr_of_mut!(view_size),
                    ViewUnmap,
                    0x2000,
                    PAGE_READWRITE,
                ) as u32
            });
            dbg!(mapview_status, view_size, base);

            //? you can actually try calling it more than once to add even more memory but i didnt bother to find how to do it reliably, sometimes it works and sometimes it doesnt

            // base = (base as usize + view_size) as _;
            // let mapview_status = NtStatus::from_u32(unsafe {
            //     NtMapViewOfSection(
            //         section,
            //         NtCurrentProcess,
            //         ptr::addr_of_mut!(base),
            //         0,
            //         0x1000,
            //         ptr::null_mut(),
            //         ptr::addr_of_mut!(view_size),
            //         ViewUnmap,
            //         0x2000,
            //         PAGE_READWRITE,
            //     ) as u32
            // });
            // dbg!(mapview_status, view_size, base);
        }

        loop
        {
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}
