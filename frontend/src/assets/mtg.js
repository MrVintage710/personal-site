export function get_cards(input_string) {
  let names = input_string.split('|');
  return names;
}

export function get_body(card_names) {
  let result = [];
  for(const name of card_names) {
    result.push({
      name : name.trim()
    });
  }
  return result;
}