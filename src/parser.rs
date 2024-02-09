use crate::token::Token;
use crate::Lexer;

use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, i32>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

pub fn interpret_code(lexer: &mut Lexer, environment: &mut Environment) -> Option<()> {
    loop {
        match lexer.next()? {
            Token::Seja => {
                let Token::Identificador(var_name) = lexer.next()? else {
                    panic!("Erro: Esperava-se um identificador após 'seja'");
                };

                let Token::Numero(var_value) = lexer.next()? else {
                    panic!("Erro: Esperava-se um valor após o identificador da declaração 'seja'");
                };

                environment.variables.insert(var_name, var_value);
            }
            Token::Identificador(name) => match lexer.next()? {
                Token::Incremento => {
                    let value = environment
                        .variables
                        .get_mut(&name)
                        .unwrap_or_else(|| panic!("Variável não definida: {}", name));
                    *value += 1;
                }
                Token::Decremento => {
                    let value = environment
                        .variables
                        .get_mut(&name)
                        .unwrap_or_else(|| panic!("Variável não definida: {}", name));
                    *value -= 1;
                }
                Token::AtribuicaoIncremento => match lexer.next()? {
                    Token::Identificador(var2) => {
                        let target = if let Some(target) = environment.variables.get(&var2) {
                            *target
                        } else {
                            panic!("Erro: Variavel inexperada '{name}'")
                        };

                        let value = environment
                            .variables
                            .get_mut(&name)
                            .unwrap_or_else(|| panic!("Variável não definida: {}", name));

                        *value += target;
                    }
                    Token::Numero(target) => {
                        let value = environment
                            .variables
                            .get_mut(&name)
                            .unwrap_or_else(|| panic!("Variável não definida: {}", name));

                        *value += target;
                    }
                    _ => panic!("Erro: Token inválido após operação de atribuição incremento"),
                },
                _ => todo!(),
            },
            Token::Imprima => match lexer.next()? {
                Token::Identificador(name) => match environment.variables.get(&name) {
                    Some(value) => println!("{value}"),
                    None => panic!("Erro: Variavel inexperada '{name}'"),
                },
                Token::Numero(value) => println!("{value}"),
                _ => panic!("Erro: Token inválido após imprima"),
            },
            Token::Enquanto => {
                let Token::Identificador(condition_var_name) = lexer.next()? else {
                    panic!("Erro: Esperava-se um identificador após 'enquanto'");
                };

                let Token::Operador(operation) = lexer.next()? else {
                    panic!("Erro: Esperava-se um operador após o identificador na condição do 'enquanto'");
                };

                let Token::Numero(condition_value) = lexer.next()? else {
                    panic!("Erro: Esperava-se um número após o operador na condição do 'enquanto'");
                };

                let Token::Faca = lexer.next()? else {
                    panic!(
                        "Erro: Esperava-se token 'faça' após expressão de condição do 'enquanto'"
                    );
                };

                let enquanto_init = lexer.pos;

                let value = *environment.variables.get(&condition_var_name)?;
                match operation.as_str() {
                    "<" => {
                        if value >= condition_value {
                            continue;
                        }
                    }
                    ">" => {
                        if value <= condition_value {
                            continue;
                        }
                    }
                    "=" => {
                        if value != condition_value {
                            continue;
                        }
                    }
                    _ => panic!("Operador inválido na condição do 'enquanto'"),
                }

                // Avaliar a condição enquanto o loop estiver em execução
                loop {
                    interpret_code(lexer, environment)?;

                    let value = *environment.variables.get(&condition_var_name)?;

                    match operation.as_str() {
                        "<" => {
                            if value >= condition_value {
                                break;
                            }
                        }
                        ">" => {
                            if value <= condition_value {
                                break;
                            }
                        }
                        "=" => {
                            if value != condition_value {
                                break;
                            }
                        }
                        _ => panic!("Operador inválido na condição do 'enquanto'"),
                    }

                    lexer.pos = enquanto_init;
                }
            }
            Token::Fim => break,
            _ => todo!(),
        }
    }

    Some(())
}
