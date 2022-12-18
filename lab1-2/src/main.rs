#![allow(dead_code)]
#![allow(unused)]

use std::collections::{BTreeMap, HashMap, VecDeque};
use tabled::{Tabled, Table, Style, Alignment, ModifyObject};
use tabled::builder::Builder;
use tabled::object::Rows;

fn main() {
    // реалізація матричного шифру з стовпцевим і рядковим ключами
    println!("Result: {}",
             crypto_word(
                 String::from("програмне забезпечення"), // слово що шифрується
                 6, // кількість колонок в матриці
                 String::from("крипто"), // горизонтальний ключ
                 String::from("шифр"))); // вертикальний ключ

    println!();
    println!();

    // шифр для другого варіанту квадрата Полібія
    println!("============== Poliby Square =============");
    println!("Result: {}",
             poliby_square(
                 String::from("абвгдеєжзиійклмнопрстуфхцчшьюя"), // алфавіт
                 6)); // кількість колонок в матриці
}

// -------------------------------- Функції для квадрата Полібія ---------------------------------
fn poliby_square(key: String, number_of_columns: i32) -> String {
    let mut matrix: Vec<Vec<char>> = vec![];
    push_to_matrix(&mut matrix, key, number_of_columns); // генерим матрицю з алфавіту
    print_matrix(&matrix);
    poliby_crypt(&mut matrix, String::from("заміна")) // функція шифрування Полібі
}
fn poliby_crypt(matrix: &mut Vec<Vec<char>>, word_to_crypt: String) -> String
{
    // ініціалізація данних
    let mut crypt_table: Vec<Vec<u8>> = Vec::new();

    let mut horizontal_coords: Vec<u8> = Vec::new();
    let mut vertical_coords: Vec<u8> = Vec::new();

    // пошук горизонтальних і вертикальних координат
    for (index, char) in word_to_crypt.chars().enumerate() {
        if let Some(result) = find_in_matrix_for_poliby_crypt(matrix.clone(),char) {
            horizontal_coords.push(result.0 as u8);
            vertical_coords.push(result.1 as u8);
        }
    }
    // запис горизонтальних і вертикальних координатів в 1 матрицю
    crypt_table.push(horizontal_coords.clone());
    crypt_table.push(vertical_coords.clone());


    println!("Horizontal coords:\t {:?}",horizontal_coords);
    println!("Vertical coords:\t {:?}",vertical_coords);

    // перетворення горизонтальних і вертикальних координат в масиви по 2 цифри читаючи по рядкам
    let mut indexes_of_crypt:Vec<Vec<u8>> = Vec::new();

    for (index,vec) in crypt_table.iter().enumerate() {
        let mut tmp:Vec<u8> = Vec::new();
        for item in vec {
            tmp.push(*item);
            if tmp.len() == 2 {
                indexes_of_crypt.push(tmp.clone());
                tmp = Vec::new();
            }
        }
    }

    println!("По дві цифри читаючи по рядкам: {:?}", indexes_of_crypt);

    // пошук символів(букв) в matrix(алфавіт) по нових координатах і запис їх в result
    let mut result = String::new();
    for indexes in indexes_of_crypt {
        result.push(matrix[(indexes[0] - 1) as usize][(indexes[1] - 1) as usize]);
    }
    // повернення результату
    result
}
fn find_in_matrix_for_poliby_crypt(matrix: Vec<Vec<char>>, char: char) -> Option<(usize,usize)>
{
    // пошук координатів і повернення кортежа(tuple) з вертикальним і горизонтальнмм індексом + 1
    for (vertical_index,item) in matrix.iter().enumerate() {
        for (horizontal_index,ch) in item.iter().enumerate() {
            if *ch == char {
                return Some((vertical_index + 1,horizontal_index + 1));
            }
        }
    }
    None
}
// ------------------------------------------------------------------------------------------------


fn crypto_word(key: String, number_of_columns: i32, row_key: String, column_key: String) -> String
{
    let mut matrix: Vec<Vec<char>> = vec![];

    println!("Creating of matrix: ");
    push_to_matrix(&mut matrix, key, number_of_columns);
    print_matrix(&matrix);

    println!("Added key `{}`: ", &row_key);
    insert_word_to_matrix(&mut matrix, row_key.clone(), number_of_columns);
    print_matrix(&matrix);

    println!("Sorted columns by key `{}`: ", &row_key);
    sort_alphabetically_of_key(&mut matrix);
    print_matrix(&matrix);

    println!("Added key `{}`: ", &column_key);
    insert_word_to_matrix_in_column(&mut matrix, column_key.clone());
    print_matrix(&matrix);

    println!("Sorted rows by key `{}`: ", &column_key);
    sort_alphabetically_of_column_key(&mut matrix);

    print_matrix(&matrix);
    get_crypted_word(matrix)
}

fn push_to_matrix(matrix: &mut Vec<Vec<char>>, key: String, number_of_columns: i32)
{
    // функція для генерації матриці з заданого слова(ключа) key з кількістю колонок number_of_columns


    let mut temp_matrix: Vec<char> = vec![];

    for (_index, char) in key.chars().enumerate() {
        if char.is_whitespace() {
            continue;
        }
        temp_matrix.push(char);
        if temp_matrix.len() == number_of_columns as usize {
            matrix.push(temp_matrix.clone());
            temp_matrix = vec![];
        }
    }

    if temp_matrix.len() > 0 {
        while temp_matrix.len() != number_of_columns as usize {
            temp_matrix.push('\0');
        }

        matrix.push(temp_matrix.clone());
    }
}


fn insert_word_to_matrix(matrix: &mut Vec<Vec<char>>, word: String, number_of_columns: i32)
{
    // функція для вставки нового слова ( горизонтально)
    if word.chars().count() <= number_of_columns as usize {
        matrix.insert(0, word.chars().collect());
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>)
{
    let mut builder = Builder::default();
    for item in matrix {
        builder.add_record(item);
    }
    let table = builder.build()
        .with(Style::rounded())
        .to_string();
    println!("{}", table);
}

fn sort_alphabetically_of_key(matrix: &mut Vec<Vec<char>>)
{
    // сортування ключа
    let mut key_sort: Vec<char> = matrix.first().unwrap().clone();
    key_sort.sort();

    // пошук нового порядку
    let mut new_order: HashMap<usize, usize> = HashMap::new();
    let key: Vec<char> = matrix.first().unwrap().clone();

    for (index, char) in key.iter().enumerate() {
        new_order.insert(index, key_sort.iter().position(|&x| x == *char).unwrap());
    }

    // перезапис колонок
    let mut new_matrix: Vec<Vec<char>> = vec![];
    for item in matrix.clone() {
        let mut row_btree: BTreeMap<usize, char> = BTreeMap::new();

        for (old_i, new_i) in new_order.iter() {
            row_btree.insert(*new_i, item[*old_i]);
        }

        new_matrix.push(row_btree.into_values().collect());
    }

    *matrix = new_matrix;
}

fn insert_word_to_matrix_in_column(matrix: &mut Vec<Vec<char>>, word: String)
{
    let mut chars: VecDeque<char> = word.chars().collect();
    // dbg!("{:?}", chars.pop_front().unwrap());
    let mut is_first = true;
    for item in matrix {
        if is_first {
            item.insert(0, '\0');
            is_first = false;
        } else {
            match chars.pop_front() {
                None => item.insert(0, '\0'),
                Some(new_char) => item.insert(0, new_char)
            }
        }
    }
}

fn sort_alphabetically_of_column_key(matrix: &mut Vec<Vec<char>>)
{
    // берем ключ який записаний в колонку і робим вектор
    let mut column_key: Vec<char> = Vec::new();
    for i in matrix.clone() {
        column_key.push(i[0]);
    }

    // сортуєм вектор з ключем в колонку
    let mut sorted_column_key = column_key.clone();
    sorted_column_key.sort();

    // шукємо новий порядок елементів
    let mut new_order: HashMap<usize, usize> = HashMap::new();
    for (index, char) in column_key.iter().enumerate() {
        new_order.insert(index, sorted_column_key.iter().position(|&x| x == *char).unwrap());
    }

    // формуєм нову матрицю з переставленими рядками
    let mut new_matrix: Vec<Vec<char>> = vec![];
    let mut row_btree: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    // структура для new_order = old_index => new_position
    for (old_i, new_i) in new_order.iter() {
        // вставка нової позиції яка знаходиться по old_index
        row_btree.insert(*new_i, matrix[*old_i].clone());
    }

    // перезапис порядку рядків
    for new_row in row_btree.into_values() {
        new_matrix.push(new_row);
    }

    *matrix = new_matrix;
}

fn get_crypted_word(matrix: Vec<Vec<char>>) -> String
{
    // формування зашифрованого повідомлення з матриці
    let mut result = String::new();
    let mut is_first = true;
    for item in matrix {
        if is_first {
            is_first = false;
            continue;
        } else {
            result.push_str(&format!("{} ", item[1..].into_iter().collect::<String>()));
        }
    }
    result
}