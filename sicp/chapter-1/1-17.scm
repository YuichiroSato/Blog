(define (mult b n)
  (define (mult-iter i acc1 acc2)
    (cond ((<= i 1) (+ acc1 acc2))
          ((even? i) (mult-iter (/ i 2) (* acc1 2) acc2))
          (else (mult-iter (- i 1) acc1 (+ acc1 acc2)))))
  (if (<= n 1)
      b
      (mult-iter (- n 1) b b)))
