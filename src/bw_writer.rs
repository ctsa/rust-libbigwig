use std::ffi::CString;

use simple_error::*;

use super::inc_bindings::*;

pub struct BigWigWriter {
    bw_fp: *mut bigWigFile_t,
}

impl BigWigWriter {
    /// Open a bigwig file and write header information
    ///
    /// File will be completed when this object goes out of scope.
    ///
    pub fn new(filename: &str, chrom_names: &[String], chrom_lens: &[u32]) -> SimpleResult<Self> {
        assert!(!filename.is_empty());
        assert_eq!(
            chrom_names.len(),
            chrom_lens.len(),
            "Invalid BigWigWriter::new() parameters - \
        chrom name array length ({}) and chrom size array length ({}) do not match. \
        filename: `{}`",
            chrom_names.len(),
            chrom_lens.len(),
            filename
        );
        assert!(
            !chrom_names.is_empty(),
            "Invalid BigWigWriter::new() parameters - \
            No chromosome names specified. filename: `{}`",
            filename
        );

        let mut bw_writer = Self {
            bw_fp: std::ptr::null_mut(),
        };

        let bw_init_retval = unsafe { bwInit(1 << 17) };
        if bw_init_retval != 0 {
            bail!(
                "Error in libBigWig bwInit. bwInit return value: {}",
                bw_init_retval
            );
        }

        let c_filename = CString::new(filename).unwrap().into_raw();
        let c_mode = CString::new("w").unwrap();

        unsafe {
            bw_writer.bw_fp = bwOpen(c_filename, None, c_mode.as_ptr());
            // retake pointer to free memory
            let _ = CString::from_raw(c_filename);
        }

        if bw_writer.bw_fp.is_null() {
            bail!("Error occurred while opening '{}' for writing", filename);
        }

        //Allow up to 10 zoom levels, though fewer will be used in practice
        let bw_create_hdr_retval = {
            let zoom_levels = 10;
            unsafe { bwCreateHdr(bw_writer.bw_fp, zoom_levels) }
        };

        if bw_create_hdr_retval != 0 {
            bail!(
                "Error in libBigWig bwCreateHdr. bwCreateHdr return value: {}",
                bw_create_hdr_retval
            );
        }

        //Create the chromosome lists
        let mut chrom_cnames = Vec::new();
        for chrom_name in chrom_names.iter() {
            chrom_cnames.push(CString::new(chrom_name.as_str()).unwrap().into_raw());
        }
        unsafe {
            let bw_fp = bw_writer.bw_fp.as_mut().unwrap();
            bw_fp.cl = bwCreateChromList(
                chrom_cnames.as_mut_ptr(),
                chrom_lens.to_owned().as_mut_ptr(),
                chrom_cnames.len() as i64,
            );
            if bw_fp.cl.is_null() {
                bail!("Error in libBigWig bwCreateChromList");
            }
            for chrom_cname in chrom_cnames {
                // retake pointer to free memory
                let _ = CString::from_raw(chrom_cname);
            }
        };

        //Write the header
        let bw_write_hdr_retval = unsafe { bwWriteHdr(bw_writer.bw_fp) };
        if bw_write_hdr_retval != 0 {
            bail!(
                "Error in libBigWig bwWriteHdr. bwWriteHdr return value: {}",
                bw_write_hdr_retval
            );
        }
        Ok(bw_writer)
    }

    /// Add a 'fixed-step' wiggle block to the file
    ///
    pub fn add_interval_span_steps(
        &mut self,
        chrom_name: &str,
        start_pos: u32,
        span_size: u32,
        step_size: u32,
        values: &mut [f32],
    ) -> SimpleResult<()> {
        let c_chrom_name = CString::new(chrom_name).unwrap().into_raw();

        let retval = unsafe {
            bwAddIntervalSpanSteps(
                self.bw_fp,
                c_chrom_name,
                start_pos,
                span_size,
                step_size,
                values.as_mut_ptr(),
                values.len() as u32,
            )
        };

        // retake pointer to free memory
        let _ = unsafe { CString::from_raw(c_chrom_name) };

        if retval != 0 {
            bail!(
                "Error in libBigWig bwAddIntervalSpanSteps.\
             bwAddIntervalSpanSteps return value: {} chrom_name: {}",
                retval,
                chrom_name,
            );
        }

        Ok(())
    }

    /// Add a 'variable-step' wiggle block to the file
    ///
    pub fn add_interval_spans(
        &mut self,
        chrom_name: &str,
        start_pos: &mut [u32],
        span_size: u32,
        values: &mut [f32],
    ) -> SimpleResult<()> {
        assert_eq!(start_pos.len(), values.len());

        let c_chrom_name = CString::new(chrom_name).unwrap().into_raw();

        let retval = unsafe {
            bwAddIntervalSpans(
                self.bw_fp,
                c_chrom_name,
                start_pos.as_mut_ptr(),
                span_size,
                values.as_mut_ptr(),
                values.len() as u32,
            )
        };

        // retake pointer to free memory
        let _ = unsafe { CString::from_raw(c_chrom_name) };

        if retval != 0 {
            bail!(
                "Error in libBigWig bwAddIntervalSpans.\
             bwAddIntervalSpans return value: {} chrom_name: {}",
                retval,
                chrom_name,
            );
        }

        Ok(())
    }
}

impl Drop for BigWigWriter {
    fn drop(&mut self) {
        unsafe {
            if !self.bw_fp.is_null() {
                bwClose(self.bw_fp);
            }
            bwCleanup();
        }
    }
}
