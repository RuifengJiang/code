/**!
 * rainfall
 *
 * Reads a sequence of rainfall measurements from the standard input and
 * writes a summary to the standard output.
 *
 * INPUT
 *
 * The input format is a sequence of measurements represented as
 * unitless, non-negative numbers, written in ASCII, one per line:
 *
 *     12.5
 *     18
 *     7
 *     0
 *     4
 *
 * Any noise in the input---blank lines, non-numeric text, negative
 * numbers---should be ignored:
 *
 *     seven
 *     -9
 *
 * The input terminates with either end-of-file or a line "999".
 *
 * OUTPUT
 *
 * The program computes three quantities: the mean (valid) measurement,
 * the count of measurements in the interval [mean - 5, mean), and the
 * count of measurements in the interval (mean, mean + 5]. It prints the
 * results in this format:
 *
 *     Mean rainfall: 8.3 cm
 *     Below count:   2
 *     Above count:   1
 *
 * ASSUMPTIONS
 *
 *  - Numbers are read according to the language’s number reading
 *    routines, in particular the trait FromStr for type f64. This means
 *    that scientific notation ("3.4E22") is accepted, but hex is not.
 *
 *  - A line containing more than one number is noise and should be
 *    ignored.
 *
 *  - The terminator is a line of text "999", not a line of text that
 *    when interpreted is merely the number 999.0.
 *
 *  - Input must be perfect---even leading or trailing spaces make a
 *    line considered garbage.
 *
 *  - If there are no measurements to read then there is no mean value
 *    to print, so we will print an explanatory message instead.
 */

use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};

fn main() {
    let measurements = read_measurements(stdin());
    write_output(stdout(), &calculate_results(&measurements));
}

struct Results {
    mean:  f64,
    above: usize,
    below: usize,
}

fn read_measurements<R: Read>(reader: R) -> Vec<f64> {
    let mut measurements: Vec<f64> = vec![]; // Vec::new()
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}

        if let Ok(f) = line.parse() {
            if f >= 0.0 {
                measurements.push(f);
            }
        }
    }

    return measurements;
}

#[cfg(test)]
mod read_measurements_tests {
    use super::read_measurements;
    use std::io::Cursor;

    #[test]
    fn reads_three_measurements() {
        assert_read(&[3., 4., 5.], "3\n4\n5\n");
    }

    #[test]
    fn discards_negative() {
        assert_read(&[3., 4., 5.], "3\n4\n-6\n5\n");
    }

    #[test]
    fn discards_noise() {
        assert_read(&[3., 4., 5.], "3\n4\nsix\n5\n");
    }

    #[test]
    fn stops_at_999() {
        assert_read(&[3., 4.], "3\n4\n999\n5\n");
    }

    fn assert_read(expected: &[f64], input: &str) {
        let mock_read = Cursor::new(input);
        let measurements = read_measurements(mock_read);
        assert_eq!(expected.to_owned(), measurements);
    }
}

fn calculate_results(fs: &[f64]) -> Results {
    let m = mean(fs);
    let b = fs.iter().filter(|&&x| m - 5.0 <= x && x < m).count();
    let a = fs.iter().filter(|&&x| m < x && x <= m + 5.0).count();

    Results {
        mean:  m,
        above: a,
        below: b,
    }
}

#[cfg(test)]
mod calculate_results_tests {
    use super::calculate_results;

    #[test]
    fn given_example() {
        let samples = [12.5, 18., 7., 0., 4.];
        let result = calculate_results(&samples);
        assert_eq!(8.3, result.mean);
        assert_eq!(1, result.above);
        assert_eq!(2, result.below);
    }
}

fn mean(samples: &[f64]) -> f64 {
    sum(samples) / (samples.len() as f64)
}

#[cfg(test)]
mod mean_tests {
    use super::mean;

    #[test]
    fn mean_empty_is_nan() {
        assert!(mean(&[]).is_nan());
    }

    #[test]
    fn mean_2_3_4_is_3() {
        assert_eq!(3.0, mean(&[2., 3., 4.]));
    }
}

fn sum(samples: &[f64]) -> f64 {
    samples.iter().fold(0.0, |a,b| a + *b)
}

#[cfg(test)]
mod sum_tests {
    use super::sum;

    #[test]
    fn sum_empty_is_0() {
        assert_eq!(0.0, sum(&[]));
    }

    #[test]
    fn sum_1_2_3_4_is_10() {
        assert_eq!(10.0, sum(&[1., 2., 3., 4.]));
    }
}

fn write_output<W: Write>(mut writer: W, r: &Results) {
  if r.mean.is_nan() {
      write!(writer, "No measurements provided.\n").unwrap();
  } else {
      write!(writer, "Mean rainfall: {} cm\n", r.mean).unwrap();
      write!(writer, "Below count:   {}\n", r.below).unwrap();
      write!(writer, "Above count:   {}\n", r.above).unwrap();
  }
}

#[cfg(test)]
mod write_output_tests {
    use super::{write_output, Results};
    use std::io::Cursor;

    #[test]
    fn no_measurements_output() {
        use std::f64::NAN;
        assert_write("No measurements provided.\n",
                     &Results { mean:  NAN, above: 0, below: 0 });
    }

    #[test]
    fn some_measurements_output() {
        assert_write(
            "Mean rainfall: 5 cm\nBelow count:   3\nAbove count:   2\n",
            &Results { mean:  5., above: 2, below: 3 });
    }

    fn assert_write(expected: &str, results: &Results) {
        let mut writer = Cursor::new(vec![]);
        write_output(&mut writer, results);
        assert_eq!(expected.as_bytes(), &*writer.into_inner());
    }
}
