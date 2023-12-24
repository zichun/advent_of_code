#![allow(non_snake_case)]

use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Hail {
    coord: (isize, isize, isize),
    vel: (isize, isize, isize),
}

#[aoc_generator(day24)]
fn parse(inp: &str) -> Vec<Hail> {
    inp.lines().map(|l| {
        let mut l = l.split(" @ ");
        let mut c = l.next().unwrap().split(", ");
        let mut v = l.next().unwrap().split(", ");
        Hail {
            coord: (c.next_token(), c.next_token(), c.next_token()),
            vel: (v.next_token(), v.next_token(), v.next_token()),
        }
    }).collect()
}

fn get_mc(h: &Hail) -> (f64, f64) {
    let m = (h.vel.1 as f64) / (h.vel.0 as f64);
    let c = h.coord.1 as f64 - m * h.coord.0 as f64;
    (m, c)
}
fn solve_p1(h0: &Hail, h1: &Hail, min_dim: isize, max_dim: isize) -> bool {
    let (m0, c0) = get_mc(h0);
    let (m1, c1) = get_mc(h1);
    if m0 == m1 {
        false
    } else {
        let x = (c0 - c1) / (m1 - m0);
        let y = m0 * x + c0;
        let t0 = ((x - h0.coord.0 as f64) as isize) / h0.vel.0;
        let t1 = ((x - h1.coord.0 as f64) as isize) / h1.vel.0;
        t1 > 0 && t0 > 0 &&
            (min_dim as f64 <= x && x <= max_dim as f64) &&
            (min_dim as f64 <= y && y <= max_dim as f64)
    }
}

#[aoc(day24, part1)]
fn part1(inp: &[Hail]) -> usize {
    let mut ans = 0;
    for i in 0..inp.len() {
        for j in i + 1..inp.len() {
            if solve_p1(&inp[i], &inp[j], 200000000000000, 400000000000000) {
                ans += 1;
            }
        }
    }
    ans
}

fn is_natural(f: f64) -> bool {
    f > 0.0 && f.ceil() == f.floor()
}

#[aoc(day24, part2)]
fn part2(inp: &[Hail]) -> isize {
    let h0 = inp[1];
    let h1 = inp[2];
    let h2 = inp[3];

    // factoring out t
    // (h0.coord.0 - x) / (dx - h0.vel.0) == (h0.coord.1 - y) / (dy - h0.vel.1) == (h0.coord.2 - z) / (dz - h0.vel.2)
    // (h1.coord.0 - x) / (dx - h0.vel.0) == (h1.coord.1 - y) / (dy - h1.vel.1) == (h1.coord.2 - z) / (dz - h1.vel.2)
    // (h2.coord.0 - x) / (dx - h0.vel.0) == (h2.coord.1 - y) / (dy - h2.vel.1) == (h2.coord.2 - z) / (dz - h2.vel.2)

    // factoring out x
    // x - h0.coord0 == (y - h0.coord.1)(h0.vel.0 - dx)/(h0.vel.1 - dy) == (z - h0.coord.2)(h0.vel.0 - dx)/(h0.vel.2 - dz)
    // x - h1.coord0 == (y - h1.coord.1)(h1.vel.0 - dx)/(h1.vel.1 - dy) == (z - h1.coord.2)(h1.vel.0 - dx)/(h1.vel.2 - dz)
    // x - h2.coord0 == (y - h2.coord.1)(h2.vel.0 - dx)/(h2.vel.1 - dy) == (z - h2.coord.2)(h2.vel.0 - dx)/(h2.vel.2 - dz)


    fn calc0(dx: isize, a: isize, b: isize, c: isize, d: isize, ap: isize, bp: isize, cp: isize, dp: isize) -> (isize, isize, isize, isize, isize) {
        let Y = a;
        let DX = b;
        let DY = c;
        let X = d;
        let Yp = ap;
        let DXp = bp;
        let DYp = cp;
        let Xp = dp;

        /*
        (DX - dx) * (DYp - dy) * y + Y * DYp * dx - Y * dx * dy - X * DYp * dy + X * dy * dy + X * DY * DYp - X * DY * dy - Y * DX * DYp + Y * DX * dy ==
        (DXp - dx) * (DY - dy) * y + (DY - dy) * Yp * dx - (DY - dy) * Xp * dy + Xp * DYp * DY - Xp * DYp * dy - Yp * DXp * DY + Yp * DXp * dy
         */

        /*
        println!("A {}", Y * DYp * dx - Y * dx * dy - X * DYp * dy + X * dy * dy + X * DY * DYp - X * DY * dy - Y * DX * DYp + Y * DX * dy);
        println!("B {}", Yp * DY * dx - Yp * dx * dy - Xp * DY * dy + Xp * dy * dy + Xp * DYp * DY - Xp * DYp * dy - Yp * DXp * DY + Yp * DXp * dy);
        println!("C {}", (DXp - dx) * (DY - dy) * y - (DX - dx) * (DYp - dy) * y);
         */

        let q = Y * DYp * dx + X * DY * DYp - Y * DX * DYp - (Yp * DY * dx + Xp * DYp * DY - Yp * DXp * DY);
        let w = -Y * dx - X * DYp - X * DY + Y * DX - (-Yp * dx - Xp * DY - Xp * DYp + Yp * DXp);
        let e = X - Xp;
        let r = DXp * DY - DY * dx - (DX * DYp - DYp * dx);
        let t = DX - DXp;
        //            println!("{} {} {} {} {}", q, w, e, r, t);
        //            println!("{} {}", (q + w + e), (r + t));

        (q, w, e, r, t)
    }
    fn solve_dy(q0: isize, w0: isize, e0: isize, r0: isize, t0: isize,
                q1: isize, w1: isize, e1: isize, r1: isize, t1: isize) -> Vec<isize> {
        let a0 = e0 * t1;
        let b0 = e0 * r1 + w0 * t1;
        let c0 = q0 * t1 + w0 * r1;
        let d0 = q0 * r1;
        let a1 = e1 * t0;
        let b1 = e1 * r0 + w1 * t0;
        let c1 = q1 * t0 + w1 * r0;
        let d1 = q1 * r0;
        let a = a0 - a1;
        let b = b0 - b1;
        let c = c0 - c1;
        let d = d0 - d1;

        (-400..=400).filter(|dy| a * dy * dy * dy + b * dy * dy + c * dy + d == 0).collect::<Vec<_>>()
    }

    fn get_solns<F: Fn((isize, isize, isize)) -> isize, G: Fn((isize, isize, isize)) -> isize>(d: isize, h0: &Hail, h1: &Hail, h2: &Hail, left: F, right: G) -> Vec<(isize, isize, isize)> {
        let (q0, w0, e0, r0, t0) = calc0(d, right(h0.coord), left(h0.vel), right(h0.vel), left(h0.coord),
                                         right(h1.coord), left(h1.vel), right(h1.vel), left(h1.coord));
        let (q1, w1, e1, r1, t1) = calc0(d, right(h1.coord), left(h1.vel), right(h1.vel), left(h1.coord),
                                         right(h2.coord), left(h2.vel), right(h2.vel), left(h2.coord));
        let (q2, w2, e2, r2, t2) = calc0(d, right(h0.coord), left(h0.vel), right(h0.vel), left(h0.coord),
                                         right(h2.coord), left(h2.vel), right(h2.vel), left(h2.coord));
        // (Q + Wdy + Tdy^2) / (E + Rdy) is constant

        let dy0 = solve_dy(q0, w0, e0, r0, t0, q1, w1, e1, r1, t1);
        let dy1 = solve_dy(q1, w1, e1, r1, t1, q2, w2, e2, r2, t2);
        let dy2 = solve_dy(q0, w0, e0, r0, t0, q2, w2, e2, r2, t2);

        dy0.iter().filter(|s| {
            dy1.contains(s) && dy2.contains(s)// && r0 + t0 * (**s) != 0
        })
            .copied()
            .map(|dy| {
                let y = if r2 + t2 * dy != 0 {
                    (q2 + w2 * dy + e2 * dy * dy) / (r2 + t2 * dy)
                } else if r1 + t1 * dy != 0 {
                    (q1 + w1 * dy + e1 * dy * dy) / (r1 + t1 * dy)
                } else {
                    (q0 + w0 * dy + e0 * dy * dy) / (r0 + t0 * dy)
                };
                let x = if (right(h0.vel) - dy) != 0 {
                    (y - right(h0.coord)) * (left(h0.vel) - d) / (right(h0.vel) - dy) + left(h0.coord)
                } else if (right(h1.vel) - dy) != 0 {
                    (y - right(h1.coord)) * (left(h1.vel) - d) / (right(h1.vel) - dy) + left(h1.coord)
                } else {
                    (y - right(h2.coord)) * (left(h2.vel) - d) / (right(h2.vel) - dy) + left(h2.coord)
                };
                (x, y, dy)
            })
            .collect::<Vec<_>>()
    }

    for dx in -400..=400 {
        // // A is .coord.1, B is .vel.0, C is .vel.1, D is coord.0
        // let (q0, w0, e0, r0, t0) = calc0(dx, h0.coord.1, h0.vel.0, h0.vel.1, h0.coord.0,
        //                             h1.coord.1, h1.vel.0, h1.vel.1, h1.coord.0);
        // let (q1, w1, e1, r1, t1) = calc0(dx, h1.coord.1, h1.vel.0, h1.vel.1, h1.coord.0,
        //                             h2.coord.1, h2.vel.0, h2.vel.1, h2.coord.0);
        // let (q2, w2, e2, r2, t2) = calc0(dx, h2.coord.1, h2.vel.0, h2.vel.1, h2.coord.0,
        //                             h0.coord.1, h0.vel.0, h0.vel.1, h0.coord.0);
        // // (Q + Wdy + Tdy^2) / (E + Rdy) is constant
        // let dy0 = solve_dy(q0, w0, e0, r0, t0, q1, w1, e1, r1, t1);
        // let dy1 = solve_dy(q1, w1, e1, r1, t1, q2, w2, e2, r2, t2);
        // let dy2 = solve_dy(q0, w0, e0, r0, t0, q2, w2, e2, r2, t2);

        // let solns = dy0.iter().filter(|s| {
        //     dy1.contains(s) && dy2.contains(s) && r0 + t0 * (**s) != 0
        // })
        //     .copied()
        //     .collect::<Vec<_>>();

        let solns = get_solns(dx, &h0, &h1, &h2, |a| a.0, |b| b.1);
        let mut soln = None;
        solns.into_iter().for_each(|(x, y, dy)| {
            let zsolns = get_solns(dy, &h0, &h1, &h2, |a| a.1, |b| b.2);
            let inner = zsolns.iter()
                .filter(|(zsoln_y, _zsoln_z, zsoln_dz)| {
                    *zsoln_y == y
                })
                .flat_map(|(_zsoln_y, zsoln_z, zsoln_dz)| {
                    let zsolns2 = get_solns(dx, &h0, &h1, &h2, |a| a.0, |b| b.2);
                    zsolns2.into_iter().filter(|(zsoln2_x, zsoln2_z, zsoln2_dz)| {
                        *zsoln2_x == x && *zsoln2_z == *zsoln_z && *zsoln2_dz == *zsoln_dz
                    }).map(|_| {
                        (x, y, *zsoln_z, dx, dy, *zsoln_dz)
                    })
                }).collect::<Vec<_>>();
            if !inner.is_empty() {
                println!("{:?}", inner);
                soln = Some(inner[0].0 + inner[0].1 + inner[0].2);
            }
        });
        if let Some(soln) = soln {
            return soln;
        }
    }
    0
}
