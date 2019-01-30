(define (squared-sum x y)
  (+ (* x x) (* y y)))

(define (minimum? a b c)
  (and (<= a b) (<= a c)))

(define (squared-sum-of-greater a b c)
  (cond ((minimum? a b c) (squared-sum b c))
        ((minimum? b a c) (squared-sum a c))
        (else (squared-sum a b))))
