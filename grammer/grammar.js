module.exports = grammar({
  name: 'secureflow',
  rules: {
    source_file: $ => repeat($._statement),
    _statement: $ => choice($.print_statement),
    print_statement: $ => seq("print", "(", $.string, ");"),
    string: $ => /"[^"]*"/
  }
});
