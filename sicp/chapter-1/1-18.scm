(define (mul a b)
  (define (divide x)
    (if (even? x)
        (/ x 2)
        (truncate (/ x 2))))
  (define (iter x y acc)
    (cond ((= y 0) acc)
          ((odd? y) (iter (* x 2) (divide y) (+ x acc)))
          (else (iter (* x 2) (divide y) acc))))
  (iter a b 0))
