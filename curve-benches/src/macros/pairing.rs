#[macro_export]
macro_rules! pairing_bench {
    ($curve:ident, $pairing_field:ident) => {

        fn bench_pairing(c: &mut $crate::criterion::Criterion) {
            let mut group = c.benchmark_group("Pairing for ".to_string() + core::stringify!($curve));
            group.bench_function("Miller Loop", miller_loop);
            group.bench_function("Final Exponentiation", final_exponentiation);
            group.bench_function("Full Pairing", full_pairing);
        }
        
        fn miller_loop(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let g1s = (0..SAMPLES).map(|_| G1::rand(&mut rng)).collect::<Vec<_>>();
            let g2s = (0..SAMPLES).map(|_| G2::rand(&mut rng)).collect::<Vec<_>>();
            let g1s = G1::batch_normalization_into_affine(&g1s);
            let g2s = G2::batch_normalization_into_affine(&g2s);
            let prepared = g1s
                .into_iter()
                .zip(g2s)
                .map(|(g1, g2)| (g1.into(), g2.into()))
                .collect::<Vec<(<$curve as PairingEngine>::G1Prepared, <$curve as PairingEngine>::G2Prepared)>>();
            let mut count = 0;
            b.iter(|| {
                let tmp = $curve::miller_loop(&[(prepared[count].0.clone(), prepared[count].1.clone())]);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        
        fn final_exponentiation(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<_> = (0..SAMPLES)
                .map(|_| {
                    (
                        G1Affine::from(G1::rand(&mut rng)).into(),
                        G2Affine::from(G2::rand(&mut rng)).into(),
                    )
                })
                .map(|(p, q)| $curve::miller_loop(&[(p, q)]))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let tmp = $curve::final_exponentiation(&v[count]);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        
        fn full_pairing(b: &mut $crate::criterion::Bencher) {
            const SAMPLES: usize = 1000;

            let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

            let v: Vec<(G1, G2)> = (0..SAMPLES)
                .map(|_| (G1::rand(&mut rng), G2::rand(&mut rng)))
                .collect();

            let mut count = 0;
            b.iter(|| {
                let tmp = $curve::pairing(v[count].0, v[count].1);
                count = (count + 1) % SAMPLES;
                tmp
            });
        }

        $crate::criterion::criterion_group!(
            name = pairing;
            config = $crate::criterion::Criterion::default()
                .sample_size(10)
                .warm_up_time(core::time::Duration::from_millis(500))
                .measurement_time(core::time::Duration::from_secs(1));
            targets = bench_pairing,
        );
    }
}
