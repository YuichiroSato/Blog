(define (gcd a b)
  (if (= b 0)
      a
      (gcd b (remainder a b))))

(define (has-a-minus a b) (< (* a b) 0))

(define (make-rat n d)
  (let* ((g (abs (gcd n d)))
         (num (abs (/ n g)))
         (den (abs (/ d g))))
    (if (has-a-minus n d)
        (cons (* -1 num) den)
        (cons num den))))

(define (numer x) (car x))

(define (denom x) (cdr x))

(define (print-rat rat)
  (format #t "~d / ~d\n" (numer rat) (denom rat)))
