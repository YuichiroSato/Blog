(define (fringe ls)
  (cond ((null? ls) ())
	((pair? (car ls)) (append (fringe (car ls)) (fringe (cdr ls))))
	(else (cons (car ls) (fringe (cdr ls))))))
