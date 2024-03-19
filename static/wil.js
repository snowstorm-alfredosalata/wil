document.addEventListener('DOMContentLoaded', (event) => {
    var head = document.getElementsByTagName('head')[0];
    var links = head.getElementsByTagName('link');

    var eval_elements = document.getElementsByTagName('e-e');
    var eval_context = {};

    for (var i = 0; i < eval_elements.length; i++) {
        eval_elements[i].expr = eval_elements[i].innerHTML
        eval_elements[i].innerHTML = ''
    }

    for (var i = 0; i < links.length; i++) {
        if (links[i].getAttribute('rel').startsWith('data/')) {
            let namespace = links[i].getAttribute('ns');
            let target = links[i].getAttribute('href');
            let update = Number.parseInt(links[i].getAttribute('update'));

            if (update !== null) {
                setInterval(
                    () =>
                    fetch(target)
                        .then((response) => response.json())
                        .then((data) => {
                            if (namespace === null) {
                                eval_context = {...eval_context, ...data}
                            }
        
                            else {
                                eval_context[namespace] = data;
                            }
        
                            let string_eval_context = ''
                            for (let key in eval_context) {
                                string_eval_context += `const ${key} = ${eval_context[key]};\n`;
                            }
        
                            for (var i = 0; i < eval_elements.length; i++) {
                                eval_elements[i].innerHTML = eval(string_eval_context + eval_elements[i].expr)
                            }
                        }),
                    update
                )
            }

           
        }
    }
});