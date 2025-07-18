pub fn page_header_001(root_path:&str,component_id:&str) -> String {
    let component_name = "page_header_001";
    let full_root_path = format!("https://eduardoos.com/{}",root_path);
    format!(
        r#"

        <div
        id="{COMPONENT_ID}_{COMPONENT_NAME}"
        class="{COMPONENT_NAME}"
        >

            <a
            id=""
            class="{COMPONENT_NAME}_logo_anchor_container"
            href="https://eduardoos.com/"
            >
                <div
                id=""
                class="{COMPONENT_NAME}_logo_anchor_container_image_container"
                >
                    <img 
                    id=""
                    class="{COMPONENT_NAME}_logo_anchor_container_image_container_image"
                    src="/personal_photo_white_720x720.webp"
                    alt="Eduardo's photo"
                    height="100%"
                    >
                </div>
            </a>

            <section
            class="{COMPONENT_NAME}_key_information"            
            >
                <p>
                    My name is Eduardo Osteicoechea
                </p>
                <p>
                    I'm interested in creating the next generation of AI-driven software, moving AEC to unspoken levels of Efficiency.
                </p>
                <p>
                    I'm a Liscenced Architect, Professional Web, Desktop, Cloud, AI Integrations & BIM Fullstack Developer, with a great interest in UI & UX design and ethics.
                </p>
                <p>
                    This website showcases my profile, services and interests.
                </p>
                <p>
                    Conctact me through this chanels:

                </p>
                
                <a href="https://wa.me/584147281033"><button>WhatsApp</button></a>
                <a href="mailto:eduardooost@gmail.com"><button>Email</button></a>

            </section>

            <nav
            id=""
            class="{COMPONENT_NAME}_navigation"
            >
                <ol
                id=""
                class="{COMPONENT_NAME}_navigation_list"
                >
                    <li
                    id=""
                    class="{COMPONENT_NAME}_navigation_list_item"
                    >
                        <a
                        id=""
                        class="{COMPONENT_NAME}_navigation_list_item_anchor"
                        href="/{ROOT_PATH}"
                        >
                            Home
                        </a>
                    </li>
                    <li
                    id=""
                    class="{COMPONENT_NAME}_navigation_list_item"
                    >
                        <a
                        id=""
                        class="{COMPONENT_NAME}_navigation_list_item_anchor"
                        href="/{ROOT_PATH}reflecting_on_the_world_of_danger"
                        >
                            Reflecting on the world of danger
                        </a>
                    </li>
                    <li
                    class="{COMPONENT_NAME}_navigation_list_item"
                    >
                        <a
                        class="{COMPONENT_NAME}_navigation_list_item_anchor"
                        href="/{ROOT_PATH}articles/create"
                        >
                            Create Aritcles
                        </a>
                    </li>
                </ol>
            </nav>



        </div>
        <script src="page_header_001_activator_button.js"></script>
        "#,
        COMPONENT_NAME=component_name,
        COMPONENT_ID=component_id,
        ROOT_PATH=root_path
    )   
}
