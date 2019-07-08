fn plot(data: &mut (&mut [u32], usize, usize, f32), x: f32, y: f32, amount: f32) {
  if x >= 0.0 && y >= 0.0 && (x as usize) < data.1 && (y as usize) < data.2 {
    let x = x as usize;
    let y = y as usize;
    let idx = y * data.1 + x;
    data.0[idx] += (amount * data.3) as u32;
  }
}

fn fpart(n: f32) -> f32 {
  n.fract()
}

fn ipart(n: f32) -> f32 {
  if n < 0.0 {
    n.ceil()
  } else {
    n.floor()
  }
  // n as usize as f32
}

fn round(n: f32) -> f32 {
  n.round()
}

fn rfpart(n: f32) -> f32 {
  // 1.0 - n.fract()
  if n < 0.0 {
    -1.0 - n.fract()
  } else {
    1.0 - n.fract()
  }
}

fn limit(p0: &mut (f32, f32), p1: &mut (f32, f32), width: usize, height: usize) -> bool {
  let m = (p1.1 - p0.1) / (p1.0 - p0.0);
  if p0.0 < 0.0 {
    if p1.0 < 0.0 {
      return false;
    }
    let dx = -p0.0;
    p0.0 = 0.0;
    p0.1 += dx * m;
  }

  if p1.0 < 0.0 {
    let dx = -p1.0;
    p1.0 = 0.0;
    p1.1 += dx * m;
  }

  if p0.1 < 0.0 {
    if p1.1 < 0.0 {
      return false;
    }
    let dy = -p0.1;
    p0.1 = 0.0;
    p0.0 += dy / m;
  }

  if p1.1 < 0.0 {
    let dy = -p1.1;
    p1.1 = 0.0;
    p1.0 += dy / m;
  }

  return true;
}

pub fn wu(
  mut p0: (f32, f32),
  mut p1: (f32, f32),
  data: &mut [u32],
  width: usize,
  height: usize,
  full: f32,
) {
  limit(&mut p0, &mut p1, width, height);

  let steep = (p1.1 - p0.1).abs() > (p1.0 - p0.0).abs();
  let mut bitmap = (data, width, height, full);

  if steep {
    p0 = (p0.1, p0.0);
    p1 = (p1.1, p1.0);
  }

  if p0.0 > p1.0 {
    std::mem::swap(&mut p0, &mut p1);
  }

  let x0 = p0.0;
  let y0 = p0.1;
  let x1 = p1.0;
  let y1 = p1.1;

  let dx = p1.0 - p0.0;
  let dy = p1.1 - p0.1;
  let gradient = dy / dx;

  let xEnd = p0.0.round();
  let yEnd = p0.1 + gradient * (xEnd - p0.0);
  let xGap = 1.0 - (p0.0 + 0.5).fract();
  let xPixel1 = xEnd;
  let yPixel1 = yEnd as usize as f32;

  if steep {
    plot(&mut bitmap, yPixel1, xPixel1, rfpart(yEnd) * xGap);
    plot(&mut bitmap, yPixel1 + 1.0, xPixel1, fpart(yEnd) * xGap);
  } else {
    plot(&mut bitmap, xPixel1, yPixel1, rfpart(yEnd) * xGap);
    plot(&mut bitmap, xPixel1, yPixel1 + 1.0, fpart(yEnd) * xGap);
  }

  let mut intery = yEnd + gradient;
  let xEnd = round(x1);
  let yEnd = y1 + gradient * (xEnd - x1);
  let xGap = fpart(x1 + 0.5);
  let xPixel2 = xEnd;
  let yPixel2 = ipart(yEnd);

  if steep {
    plot(&mut bitmap, yPixel2, xPixel2, rfpart(yEnd) * xGap);
    plot(&mut bitmap, yPixel2 + 1.0, xPixel2, fpart(yEnd) * xGap);
  } else {
    plot(&mut bitmap, xPixel2, yPixel2, rfpart(yEnd) * xGap);
    plot(&mut bitmap, xPixel2, yPixel2 + 1.0, fpart(yEnd) * xGap);
  }

  if steep {
    // for(int x=(int)(xPixel1+1);x<=xPixel2-1;x++){
    for x in (xPixel1 + 1.0) as usize..xPixel2 as usize {
      plot(&mut bitmap, ipart(intery), x as f32, rfpart(intery));
      plot(&mut bitmap, ipart(intery) + 1.0, x as f32, fpart(intery));
      intery += gradient;
    }
  } else {
    // for(int x=(int)(xPixel1+1);x<=xPixel2-1;x++){
    for x in (xPixel1 + 1.0) as usize..xPixel2 as usize {
      plot(&mut bitmap, x as f32, ipart(intery), rfpart(intery));
      plot(&mut bitmap, x as f32, ipart(intery) + 1.0, fpart(intery));
      intery += gradient;
    }
  }
}
