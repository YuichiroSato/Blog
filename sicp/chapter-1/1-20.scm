(define (gcd a b)
  (format #t "a ~d, b ~d, remainder " a b)
  (display (format-remainder a b))
  (if (= b 0)
      a
      (gcd b (remainder a b))))

(define (format-remainder a b)
  (if (= b 0)
      (format #f "b is zero\n")
      (format #f "~d\n" (remainder a b))))
