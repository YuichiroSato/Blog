(define (product term a0 next b)
  (define (iter a1 result)
    (if (> a1 b)
        result
        (iter (next a1) (* (term a1) result))))
  (iter a0 1))

(define (factorial n)
  (product id 1 inc n))

(define (id x) x) 

(define (inc x) (+ x 1))

(define (add2 x) (+ x 2))

(define (app-pi n)
 (define (f x) (/ (* x (add2 x))
                  (* (inc x) (inc x))))
 (* 4 (product f 2.0 add2 n)))
