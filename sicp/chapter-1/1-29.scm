(define (sum term a next b)
  (if (> a b)
      0
      (+ (term a)
         (sum term (next a) next b))))

(define (inc x) (+ x 1))

(define (cube x) (* x x x))

(define (simpson f a b n)
  (define h (/ (- b a) n))
  (define (f-at m) (f (+ a (* m h))))
  (define (g m)
    (cond ((= m 0) (f-at m))
          ((= m n) (f-at m))
          ((even? m) (* 2 (f-at m)))
          (else (* 4 (f-at m)))))
  (* (/ h 3.0) (sum g 0 inc n)))
