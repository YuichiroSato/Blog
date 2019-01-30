(define (pow x n)
  (define (iter i result)
    (if (>= i n)
        result
        (iter (+ i 1) (* x result))))
  (iter 0 1))

(define (cons x y)
  (* (pow 2 x) (pow 3 y)))

(define (count z a)
  (define (iter x n)
    (if (not (= (remainder x a) 0))
        n
        (iter (/ x a) (+ n 1))))
  (iter z 0))

(define (car z) (count z 2))

(define (cdr z) (count z 3))
