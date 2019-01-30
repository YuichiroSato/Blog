(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
	  (accumulate op initial (cdr sequence)))))

(define (accumulate-n op init seqs)
  (list-list-map (lambda (ls) (accumulate op init ls)) seqs))

(define (list-list-map op seqs)
  (cond ((null? seqs) (list ()))
	((null? (cdr seqs)) (list (op (car seqs))))
	(else (cons (op (car seqs)) (list-list-map op (cdr seqs))))))

(define (dot-product v w)
  (accumulate + 0 (map * v w)))

(define (transpose mat)
  (accumulate (lambda (ls acc) (zip ls acc)) (make-empty mat) mat))

(define (zip ls1 ls2)
  (if (or (null? ls1) (null? ls2))
      ()
      (cons (cons (car ls1) (car ls2)) (zip (cdr ls1) (cdr ls2)))))

(define (flatten ls)
  (if (null? ls)
      ()
      (append (car ls) (flatten (cdr ls)))))

(define (make-empty ls)
  (if (null? ls)
      (())
      (map (lambda (x) '()) (car ls))))

(define (matrix-*-matrix m n)
  (let ((cols (transpose n)))
    (map (lambda (lsls) (list-list-map fold-tuples lsls))
	 (map (lambda (row) (zip-mat row cols)) m))))

(define (zip-mat ls mat)
  (map (lambda (a) (zip ls a)) mat))

(define (fold-tuples tuples)
  (accumulate + 0 (map (lambda (t) (* (car t) (cdr t))) tuples)))

(define matrix1 (list (list 1 2 3 4) (list 4 5 6 6) (list 6 7 8 9)))

(define matrix2 (list (list 1 2) (list 3 4) (list 5 6)))

(define matrix3 (list (list 1 1 1) (list 2 2 2)))

(define matrix4 (list (list 4 5) (list 4 5) (list 4 5)))
