use std::{thread, time};

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

pub fn indicatif() {
    /* 
    let pb = ProgressBar::new(100);
    
    for i in 0..100 {
        let ten_millis = time::Duration::from_millis(10); 
        thread::sleep(ten_millis);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    
    pb.finish_with_message("done");
    */
    
    iterator()
}

fn iterator() {
    // more examples: https://github.com/console-rs/indicatif/blob/HEAD/examples

    // Provide a custom bar style
    let pb = ProgressBar::new(1000);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
    for _ in (0..1000).progress_with(pb) {
        // ...
        thread::sleep(time::Duration::from_millis(5));
    }
}