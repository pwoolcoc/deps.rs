document.addEventListener("DOMContentLoaded", function() {
    let tabs = toArray(document.querySelectorAll(".tabs li"));
    let content = toArray(document.querySelectorAll(".sources > div"));
    if (tabs && content) {
        tabs.forEach((tab) => {
            tab.addEventListener("click", (evt) => {
                evt.preventDefault();
                let target = evt.target;
                let hash = target.hash.substring(1);
                content.forEach((div) => {
                    let id = div.getAttribute("id");
                    if (id === hash) {
                        showTab(div);
                    } else {
                        hideTab(div);
                    }
                });
                return false;
            });
        });
    }

    function hideTab(tab) {
        tab.classList.remove('is-active');
        tab.classList.add('is-hidden');
    }

    function showTab(tab) {
        tab.classList.remove('is-hidden');
        tab.classList.add('is-active');
    }

    function toArray(nodeList) {
        if (nodeList && nodeList.length > 0) {
            return Array.prototype.slice.call(nodeList);
        }
        return false;
    }
});
