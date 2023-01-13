import { element } from "svelte/internal";

export default async (blog_content_id) => {
    addEventListener("load", (event) => {
        const content_wrapper = document.getElementById(blog_content_id);
        const content = content_wrapper.querySelectorAll("h1, h2, h3, h4, h5, h6");
        construct_tree(content.values())
    })
}

function construct_tree(iterator, current_node = {level : 0, children : []}) {
    let root = current_node.root ? current_node.root : current_node;
    let parent = current_node.parent ? current_node.parent : current_node

    if(!iterator || !iterator.next || !current_node) {return root}
    
    let tag = iterator.next();
    
    if(!tag.done) {
        let level = Number(tag.id.slice(1));
    
        if(level === tag.level + 1) {
            let next = {level, children : [], root, parent : current_node, tag}
            current_node.children.push(next)
            return construct_tree(iterator, current_node = next)
        } else if(level === tag.level - 1) {
            let next = {level, children : [], root, parent : parent, tag}
        }
    } else {
        return root;
    }
}
