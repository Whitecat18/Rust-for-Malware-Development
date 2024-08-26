// Self Deletion Techniques in Rust


pub mod techniques;

use techniques::{
    create_batch_file, 
    del_CreateProcessA, 
    cmd_timeout,
    schedule_self_delete,
    delete_file_immediately,
    remote_self_deletion,
    companion_proces,
    schedule_task_deletion,
    setfileinformationbyhandle,
};

fn main(){
    // let me add the step with no's
/* 
    STEP 1: Using MoveFileExW [Schedule Deletion on Reboot]
    
    The MoveFileExW function can be used to delete the executable after it finishes running by moving it to a location 
    with the MOVEFILE_DELAY_UNTIL_REBOOT flag, which schedules it for deletion on the next reboot.
*/
    schedule_self_delete();

/* 
    STEP 2: Using Batch Script: 
    
    Simple Logic, use an batch script that deletes the executable binary after an delay. 
*/
    create_batch_file().expect("Error");  

/*
    STEP 3: Using cmd & timeout
    
    Spawn a new process using cmd with timeout to delete the file after the main program exists.
*/

    cmd_timeout();

/*
    STEP 4: Using CreateProcess API open an new cmd process and delete it !  

    start a new process with cmd.exe to delete the file
*/

    del_CreateProcessA();

/*
    STEP 5: Using SHFileOperation API for Immediate Deletion.

    This SHFileOperation API can be used to delete a file immediately without the need for a delay or reboot. 
    This method is part of the Windows Shell and can be used to send the file directly to the Recycle Bin or permanently delete 
    it.
*/
    // This may not work !
    delete_file_immediately();
    
/*
    STEP 6: Using a Remote Execution Tool
    
    This involves using a remote execution tool, such as Windows Management Instrumentation (WMI) or PowerShell, 
    to delete the file from a different process. This method makes it harder to trace the deletion back to the original executable.
    
*/
    
    remote_self_deletion();

/*
    STEP 7: Using an Secondary Process

    Using an secondary process that monitors the primary process and deletes the executable once the main process exits. 
    This technique can be useful if you need to ensure that the file is deleted immediately after execution without relying on the main process itself
*/

    companion_proces();

/*
    STEP 8: Using Windows Task Scheduler

    You can create a scheduled task that runs after a certain delay to delete the executable. This approach uses the Windows Task Scheduler, 
    which may be less suspicious than direct deletion commands.
*/

    schedule_task_deletion();


/*
    STEP 9: Using FILE_FLAG_DELETE_ON_CLOSE to Mark the File for Deletion and SetFileInformationByHandle for renaming the File !

    This method involves using a low-level Windows API call to mark the file for deletion. 
    This approach can be harder to detect and block by some security software.
*/

    setfileinformationbyhandle();

}

