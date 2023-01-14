import { element } from "svelte/internal";

export default class LeveledTree {
    constructor() {
        this.nodes = [];
    }

    push(value, level) {
        this.nodes.push({value, level})
    }

    get(index) {
        return this.nodes[index]
    }

    contains(index) {
        return index >=0 && index < this.nodes.length
    }

    get_parent(index) {
        return this.get_prior_of_level(index, this.nodes[index].level - 1).node;
    }

    get_parent_info(index) {
        if(this.contains(index)) {
            return this.get_prior_of_level(index, this.nodes[index].level - 1);
        } else {
            return null;
        }
    }

    get_parents(index) {
        let parent = this.get_parent_info(index);
        if(this.contains(index)) {
            if(parent) {
                let parents = this.get_parents(parent.index);
                parents.push(this.nodes[index].value);
                return parents
            } else {
                return [this.nodes[index].value];
            }
        }
    }

    get_last_from_pos(offset) {
        for(let i = 0; i < this.nodes.length - 1; i++) {
            let current_y = this.nodes[i].value.getBoundingClientRect().top;

            if(current_y >= offset) {
                return i - 1
            }
        }

        return -1;
    }

    get_prior_of_level(index, level, first = true) {
        if(index < 0) return null;
        if(!first) {
            let node = this.nodes[index]
            if(node.level === level) {
                return {node, index};
            }
        }

        return this.get_prior_of_level(index - 1, level, first = false);
    }
}

function check_level(levels, level) {
    if(levels[level]) {
        return levels[level].length - 1
    } else {
        levels[level] = [];
        return -1;
    }
}