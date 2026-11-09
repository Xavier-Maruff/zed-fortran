
; class objects

((program
  (program_statement)
  .
  (_)+ @class.inside
  .
  (end_program_statement))) @class.around

((module
  (module_statement)
  .
  (_)+ @class.inside
  .
  (end_module_statement))) @class.around


((submodule
  (submodule_statement)
  .
  (_)+ @class.inside
  .
  (end_submodule_statement))) @class.around

((derived_type_definition
  (derived_type_statement)
 .
  (_)+ @class.inside
  . 
  [
  (derived_type_procedures)
  (end_type_statement)
  ])) @class.around
  
((derived_type_procedures
  (contains_statement)
  .
  (_)+ @class.inside
  . 
  )) 
  
((interface
  (interface_statement)
  .
  (_)+ @class.inside
  . 
  (end_interface_statement))) @class.around

; function objects

((module_procedure
   (module_procedure_statement)
   .
   (_)+ @function.inside
   .
   (end_module_procedure_statement))) @function.around

((function
   (function_statement)
   .
   (_)+ @function.inside
   .
   (end_function_statement))) @function.around

((subroutine
   (subroutine_statement)
   .
   (_)+ @function.inside
   .
   (end_subroutine_statement))) @function.around

; entry objects (used for loop)

; Maybe the below line show the Error diagnostic, but it works as intended
((do_loop
  (_)
  . 
  (_)+ @entry.inside
  .
  (end_do_loop_statement))) @entry.around
   
; comment objects (used for "c"onditionals)

(if_statement) @comment.around

((if_statement
   (parenthesized_expression)
   . 
   (_)+ @comment.inside
   .
   [
    (elseif_clause)
    (else_clause)
    (end_if_statement)
   ]))

((elseif_clause 
  (parenthesized_expression)
  . 
  (_)+ @comment.inside
  .
  ))
  

((else_clause 
  . 
  (_)+ @comment.inside
  .
  ))

(select_case_statement) @comment.around

(select_case_statement
   (selector)
   . 
   (_)+ @comment.inside
   .
   [
   (end_select_statement)
   (case_statement)
   ]) @comment.around

((case_statement 
  (_)
  . 
  (_)+ @comment.inside
  .
  ))

(select_type_statement) @comment.around

(select_type_statement
   (selector)
   . 
   (_)+ @comment.inside
   .
   [
   (end_select_statement)
   (type_statement)
   ])

((type_statement 
  (_)
  . 
  (_)+ @comment.inside
  .
  ))

; test objects used for block

(block_construct) @test.around

((block_construct
   .
   (_)+ @test.inside
   .
   (end_block_construct_statement)))

; parameter objects 

((parameters 
   (_)+ @parameter.inside
   )) @parameter.around

