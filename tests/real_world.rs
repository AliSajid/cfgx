//! Real-world scenario tests (Category 8)
//!
//! These tests validate practical use cases, particularly for bioinformatics
//! and systems programming. All tests use feature flags for cross-platform testing.

use cfgx::cfgx;

// Test 8.1: Bioinformatics precision selection
#[test]
fn test_bioinformatics_precision() {
    cfgx! {
        if #[cfg(feature = "f64_precision")] {
            type Score = f64;
            type Probability = f64;
            const MIN_SCORE: f64 = 1e-10;
            const PRECISION_NAME: &str = "double";

            fn calculate_score(a: f64, b: f64) -> Score {
                (a * b).max(MIN_SCORE)
            }

            fn log_probability(p: Probability) -> Score {
                p.ln()
            }
        } else {
            type Score = f32;
            type Probability = f32;
            const MIN_SCORE: f32 = 1e-6;
            const PRECISION_NAME: &str = "single";

            fn calculate_score(a: f32, b: f32) -> Score {
                (a * b).max(MIN_SCORE)
            }

            fn log_probability(p: Probability) -> Score {
                p.ln()
            }
        }
    }

    // Test basic functionality
    let score = calculate_score(0.5 as Score, 0.8 as Score);
    assert!(score > MIN_SCORE);

    let prob: Probability = 0.5 as Probability;
    let log_p = log_probability(prob);
    assert!(log_p < 0.0);

    #[cfg(feature = "f64_precision")]
    assert_eq!(PRECISION_NAME, "double");

    #[cfg(not(feature = "f64_precision"))]
    assert_eq!(PRECISION_NAME, "single");
}

// Test 8.2: Platform-specific file I/O (using features instead of target_os)
#[test]
#[allow(dead_code)]
fn test_platform_file_io() {
    // Using nested if/else instead of else-if chain
    cfgx! {
        if #[cfg(feature = "os_unix")] {
            const PATH_SEPARATOR: char = '/';
            const LINE_ENDING: &str = "\n";
            const PLATFORM: &str = "unix";

            fn normalize_path(path: &str) -> String {
                path.replace('\\', "/")
            }

            fn is_absolute(path: &str) -> bool {
                path.starts_with('/')
            }
        } else {
            // Not unix - could be windows or other
            cfgx! {
                if #[cfg(feature = "os_windows")] {
                    const PATH_SEPARATOR: char = '\\';
                    const LINE_ENDING: &str = "\r\n";
                    const PLATFORM: &str = "windows";

                    fn normalize_path(path: &str) -> String {
                        path.replace('/', "\\")
                    }

                    fn is_absolute(path: &str) -> bool {
                        path.len() >= 2 && path.chars().nth(1) == Some(':')
                    }
                } else {
                    const PATH_SEPARATOR: char = '/';
                    const LINE_ENDING: &str = "\n";
                    const PLATFORM: &str = "other";

                    fn normalize_path(path: &str) -> String {
                        path.to_string()
                    }

                    fn is_absolute(path: &str) -> bool {
                        path.starts_with('/')
                    }
                }
            }
        }
    }

    // Test path operations
    #[cfg(feature = "os_unix")]
    {
        assert_eq!(PATH_SEPARATOR, '/');
        assert_eq!(normalize_path("a\\b\\c"), "a/b/c");
        assert!(is_absolute("/home/user"));
        assert!(!is_absolute("relative/path"));
    }

    #[cfg(feature = "os_windows")]
    {
        assert_eq!(PATH_SEPARATOR, '\\');
        assert_eq!(normalize_path("a/b/c"), "a\\b\\c");
        assert!(is_absolute("C:\\Users"));
        assert!(!is_absolute("relative\\path"));
    }

    #[cfg(not(any(feature = "os_unix", feature = "os_windows")))]
    {
        assert_eq!(PATH_SEPARATOR, '/');
        assert_eq!(normalize_path("test/path"), "test/path");
    }
}

// Test 8.3: Feature-gated algorithm selection
#[test]
fn test_algorithm_selection() {
    cfgx! {
        if #[cfg(feature = "simd")] {
            fn dot_product(a: &[f32], b: &[f32]) -> f32 {
                // Simulated SIMD implementation
                // In reality, this would use actual SIMD intrinsics
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| x * y)
                    .sum()
            }

            fn vector_add(a: &[f32], b: &[f32]) -> Vec<f32> {
                // Simulated SIMD implementation
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| x + y)
                    .collect()
            }

            const ALGORITHM: &str = "simd";
            const BATCH_SIZE: usize = 8;
        } else {
            fn dot_product(a: &[f32], b: &[f32]) -> f32 {
                // Scalar implementation
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| x * y)
                    .sum()
            }

            fn vector_add(a: &[f32], b: &[f32]) -> Vec<f32> {
                // Scalar implementation
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| x + y)
                    .collect()
            }

            const ALGORITHM: &str = "scalar";
            const BATCH_SIZE: usize = 1;
        }
    }

    // Test that both implementations work correctly
    let a = vec![1.0, 2.0, 3.0, 4.0];
    let b = vec![2.0, 3.0, 4.0, 5.0];

    let dot = dot_product(&a, &b);
    assert_eq!(dot, 1.0 * 2.0 + 2.0 * 3.0 + 3.0 * 4.0 + 4.0 * 5.0);

    let sum = vector_add(&a, &b);
    assert_eq!(sum, vec![3.0, 5.0, 7.0, 9.0]);

    #[cfg(feature = "simd")]
    {
        assert_eq!(ALGORITHM, "simd");
        assert_eq!(BATCH_SIZE, 8);
    }

    #[cfg(not(feature = "simd"))]
    {
        assert_eq!(ALGORITHM, "scalar");
        assert_eq!(BATCH_SIZE, 1);
    }
}

// Test 8.4: Debug vs release configurations
#[test]
fn test_debug_release_config() {
    cfgx! {
        if #[cfg(debug_assertions)] {
            const BUFFER_SIZE: usize = 128;
            const ENABLE_LOGGING: bool = true;
            const BUILD_MODE: &str = "debug";

            fn log(msg: &str) {
                eprintln!("[DEBUG] {}", msg);
            }

            fn validate_input(x: u32) -> Result<u32, &'static str> {
                if x > 1000 {
                    Err("value too large")
                } else {
                    Ok(x)
                }
            }
        } else {
            const BUFFER_SIZE: usize = 8192;
            const ENABLE_LOGGING: bool = false;
            const BUILD_MODE: &str = "release";

            fn log(_msg: &str) {
                // No-op in release
            }

            fn validate_input(x: u32) -> Result<u32, &'static str> {
                // Skip validation in release
                Ok(x)
            }
        }
    }

    // Test configuration values
    #[cfg(debug_assertions)]
    {
        assert_eq!(BUFFER_SIZE, 128);
        assert!(ENABLE_LOGGING);
        assert_eq!(BUILD_MODE, "debug");
        assert!(validate_input(2000).is_err());
    }

    #[cfg(not(debug_assertions))]
    {
        assert_eq!(BUFFER_SIZE, 8192);
        assert!(!ENABLE_LOGGING);
        assert_eq!(BUILD_MODE, "release");
        assert!(validate_input(2000).is_ok());
    }

    log("test message"); // Should work in both modes
}

// Test 8.5: Genomic data processing with different representations
#[test]
fn test_genomic_data_representation() {
    cfgx! {
        if #[cfg(feature = "compact_genome")] {
            type Nucleotide = u8;  // 2 bits per nucleotide, packed
            const BITS_PER_BASE: u8 = 2;
            const STORAGE: &str = "compact";

            fn encode_base(base: char) -> Nucleotide {
                match base {
                    'A' | 'a' => 0,
                    'C' | 'c' => 1,
                    'G' | 'g' => 2,
                    'T' | 't' => 3,
                    _ => 0,
                }
            }

            fn decode_base(encoded: Nucleotide) -> char {
                match encoded {
                    0 => 'A',
                    1 => 'C',
                    2 => 'G',
                    3 => 'T',
                    _ => 'N',
                }
            }
        } else {
            type Nucleotide = char;  // Direct char representation
            const BITS_PER_BASE: u8 = 32;  // char is 4 bytes
            const STORAGE: &str = "direct";

            fn encode_base(base: char) -> Nucleotide {
                base.to_ascii_uppercase()
            }

            fn decode_base(encoded: Nucleotide) -> char {
                encoded
            }
        }
    }

    // Test encoding/decoding
    let bases = ['A', 'C', 'G', 'T'];
    for base in bases.iter() {
        let encoded = encode_base(*base);
        let decoded = decode_base(encoded);
        assert!(decoded == *base || decoded == base.to_ascii_uppercase());
    }

    #[cfg(feature = "compact_genome")]
    {
        assert_eq!(BITS_PER_BASE, 2);
        assert_eq!(STORAGE, "compact");
    }

    #[cfg(not(feature = "compact_genome"))]
    {
        assert_eq!(BITS_PER_BASE, 32);
        assert_eq!(STORAGE, "direct");
    }
}

// Test 8.6: Statistical analysis with different numeric backends
#[test]
fn test_statistical_backends() {
    cfgx! {
        if #[cfg(feature = "high_precision_stats")] {
            type Float = f64;
            const EPSILON: f64 = 1e-15;

            fn mean(values: &[f64]) -> Float {
                let sum: f64 = values.iter().sum();
                sum / values.len() as f64
            }

            fn variance(values: &[f64]) -> Float {
                let m = mean(values);
                let sum_sq: f64 = values.iter().map(|x| (x - m).powi(2)).sum();
                sum_sq / values.len() as f64
            }
        } else {
            type Float = f32;
            const EPSILON: f32 = 1e-6;

            fn mean(values: &[f32]) -> Float {
                let sum: f32 = values.iter().sum();
                sum / values.len() as f32
            }

            fn variance(values: &[f32]) -> Float {
                let m = mean(values);
                let sum_sq: f32 = values.iter().map(|x| (x - m).powi(2)).sum();
                sum_sq / values.len() as f32
            }
        }
    }

    #[cfg(feature = "high_precision_stats")]
    {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let m = mean(&data);
        assert!((m - 3.0).abs() < EPSILON);

        let v = variance(&data);
        assert!((v - 2.0).abs() < 0.01);
    }

    #[cfg(not(feature = "high_precision_stats"))]
    {
        let data = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
        let m = mean(&data);
        assert!((m - 3.0).abs() < EPSILON);

        let v = variance(&data);
        assert!((v - 2.0).abs() < 0.01);
    }
}

// Test 8.7: Memory allocation strategies
#[test]
fn test_allocation_strategies() {
    cfgx! {
        if #[cfg(feature = "large_buffers")] {
            const DEFAULT_CAPACITY: usize = 1_000_000;
            const CHUNK_SIZE: usize = 65536;
            const STRATEGY: &str = "large";

            fn allocate_buffer() -> Vec<u8> {
                Vec::with_capacity(DEFAULT_CAPACITY)
            }
        } else {
            const DEFAULT_CAPACITY: usize = 4096;
            const CHUNK_SIZE: usize = 1024;
            const STRATEGY: &str = "small";

            fn allocate_buffer() -> Vec<u8> {
                Vec::with_capacity(DEFAULT_CAPACITY)
            }
        }
    }

    let buffer = allocate_buffer();
    assert!(buffer.capacity() >= DEFAULT_CAPACITY);

    #[cfg(feature = "large_buffers")]
    {
        assert_eq!(STRATEGY, "large");
        assert_eq!(CHUNK_SIZE, 65536);
    }

    #[cfg(not(feature = "large_buffers"))]
    {
        assert_eq!(STRATEGY, "small");
        assert_eq!(CHUNK_SIZE, 1024);
    }
}

// Test 8.8: Multi-feature combination test
#[test]
fn test_feature_combinations() {
    cfgx! {
        if #[cfg(all(feature = "high_precision", feature = "extended"))] {
            const CONFIG: &str = "high-extended";
            type ComputeFloat = f64;
        } else {
            cfgx! {
                if #[cfg(feature = "high_precision")] {
                    const CONFIG: &str = "high-standard";
                    type ComputeFloat = f64;
                } else {
                    cfgx! {
                        if #[cfg(feature = "extended")] {
                            const CONFIG: &str = "standard-extended";
                            type ComputeFloat = f32;
                        } else {
                            const CONFIG: &str = "standard";
                            type ComputeFloat = f32;
                        }
                    }
                }
            }
        }
    }

    // Verify the right config is active
    #[cfg(all(feature = "high_precision", feature = "extended"))]
    {
        assert_eq!(CONFIG, "high-extended");
        let _x: ComputeFloat = 1.0f64;
    }

    #[cfg(all(feature = "high_precision", not(feature = "extended")))]
    {
        assert_eq!(CONFIG, "high-standard");
        let _x: ComputeFloat = 1.0f64;
    }

    #[cfg(all(not(feature = "high_precision"), feature = "extended"))]
    {
        assert_eq!(CONFIG, "standard-extended");
        let _x: ComputeFloat = 1.0f32;
    }

    #[cfg(not(any(feature = "high_precision", feature = "extended")))]
    {
        assert_eq!(CONFIG, "standard");
        let _x: ComputeFloat = 1.0f32;
    }
}
