while batch_checkpoints < max_batch_checkpoints {
  let Some(entry) = pending.first_entry() else {
      break;
  };

  match next_checkpoint.cmp(entry.key()) {
      // Next pending checkpoint is from the future.
      Ordering::Less => break,

      // This is the next checkpoint -- include it.
      Ordering::Equal => {
          let indexed = entry.remove();
          batch_rows += indexed.len();
          batch_checkpoints += 1;
          handler.batch(&mut batch, indexed.values.into_iter());
          watermark = Some(indexed.watermark);
          next_checkpoint += 1;
      }

      // Next pending checkpoint is in the past, ignore it to avoid double
      // writes.
      Ordering::Greater => {
          metrics
              .total_watermarks_out_of_order
              .with_label_values(&[H::NAME])
              .inc();

          let indexed = entry.remove();
          pending_rows -= indexed.len();
      }
  }
}