        _vx = |d: DataObject<F>| -> Option<F> {
          match d {
            DataObject(d) => {
              return Some(centroid(d)[0]);
            },
            Vec(d) => {
              if d.len() > 1 {
                return Some(d[0]);
              } else {
              return None;
              }
            },
            Blank => {
              return None;
            }
          }
        }),