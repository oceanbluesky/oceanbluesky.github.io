
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! { 
            div { 
                class: "about ml-64 mt-16 mb-3",
                h2 {
                    class: "font-display font-bold text-5xl text-black",
                    "About"
                }
                p {
                    class: "font-text text-lg text-gray-500", 
                    "TEST A brief biography goes here,", 
                    br {},
                    "describing career, skills, and interests.
                    Check out what is not on this little guy's right foot, but on his head -- this was sculpted near Kunming China 1,500 years ago! Can you imagine him a retired hero dancing around a hearth, entertaining kids and village folk who have stayed up late to hear his stories?

"Happy Dancer" is the most important sculpture ever created by a human being (in my super humble opinion)...it is not violent, does not celebrate a state, nationality, or ruler -- it is not conspicuous ornate decorative or even utilitarian...it's simply human and fun. I LOVE this sculpture. It was created by a Fellow Spirit, a comrade-in-arms, an artist whose name has been lost but whose attitude resonates.

"Happy Dancer" can be found in a nondescript corner of a display case among shelves of statuettes in the Yunnan Provincial Museum, Kunming China. Completely ignored. Some day it's value will be celebrated as the Mona Lisa of sculpture.
http://en.wikipedia.org/wiki/Yunnan_Provincial_Museum

(Well, maybe not THE most important, but, one of the most important...allllthough, if forced to choooooose!! --I LOVE IT!!)

---

"Dance, when you're broken open. Dance, if you've torn the bandage off. Dance in the middle of the fighting. Dance in your blood. Dance when you're perfectly free." (Rumi)

"I would not know what the spirit of a philosopher might wish more to be than a good dancer."  Nietzsche

"I would only believe in a God that knows how to dance." Nietzsche

The "Übermensch" is the being that overcomes the "great nausea" associated with nihilism; that overcomes that most "abysmal" realization of the eternal return. He is the being that "sails over morality", and that dances over gravity (the "spirit of gravity" is Zarathustra's devil and archenemy). He is a "harvester" and a "celebrant" who endlessly affirms his existence, thereby becoming the transfigurer of his consciousness and life, aesthetically. He is initially a destructive force, excising and annihilating the insidious "truths" of the herd, and consequently reclaiming the chaos from which pure creativity is born. It is this creative force exemplified by the Übermensch that justifies suffering without displacing it in some "afterworld". 

http://en.wikipedia.org/wiki/Thus_Spoke_Zarathustra#Style

Inspiring fellow-rhapsodizers, encouraging them on to new secret paths and dancing places. Even under the influence of the narcotic draught, of which songs of all primitive men and peoples speak, or with the potent coming of spring that penetrates all nature with joy, these Dionysian emotions awake, and as they grow in intensity everything subjective vanishes into complete self-forgetfulness. In the German Middle Ages, too, singing and dancing crowds, ever increasing in number, whirled themselves from place to place under this same Dionysian impulse. [...] There are some who, from obtuseness or lack of experience, turn away from such phenomena as from "folk-diseases," with contempt or pity born of consciousness of their own "healthy-mindedness." But of course such poor wretches have no idea how corpselike and ghostly their so-called "healthy-mindedness" looks when the glowing life of the Dionysian revelers roars past them. 

"Birth of Tragedy," Nietzsche, translated by Walter Kaufmann

(alt translation: "In these dancers of Saint John and Saint Vitus we can recognize the Bacchic choruses of the Greeks, with their prehistory in Asia Minor, as far back as Babylon and the orgiastic Sacaea. Some people, either through a lack of experience or through obtuseness, turn away with pity or contempt from phenomena such as these as from 'folk diseases', bolstered by a sense of their own sanity; these poor creatures have no idea how blighted and ghostly this 'sanity' of theirs sounds when the glowing life of Dionysiac revellers thunders past them.")

http://en.wikipedia.org/wiki/The_Birth_of_Tragedy#Quotations

"If ever a breath hath come to me of the creative breath, and of the heavenly necessity which compelleth even chances to dance star-dances." Nietzsche

In Ecce Homo Nietzsche refers to the poems in the Appendix of The Gay Science, saying they were,

"written for the most part in Sicily, are quite emphatically reminiscent of the Provençal concept of gaia scienza—that unity of singer, knight, and free spirit which distinguishes the wonderful early culture of the Provençals from all equivocal cultures. The very last poem above all, "To the Mistral", an exuberant dancing song in which, if I may say so, one dances right over morality, is a perfect Provençalism."

This alludes to the birth of modern European poetry that occurred in Provence around the 13th century, whereupon, after the culture of the troubadours fell into almost complete desolation and destruction due to the Albigensian Crusade (1209–1229), other poets in the 14th century ameliorated and thus cultivated the gai saber or gaia scienza. In a similar vein, in Beyond Good and Evil Nietzsche observed that,

"love as passion—which is our European speciality—[was invented by] the Provençal knight-poets, those magnificent and inventive human beings of the "gai saber" to whom Europe owes so many things and almost owes itself. (Section 260)

Nietzsche, The Wisdom of Ecstasy" 
                }
                // img { src: "assets/images/profile.jpg", alt: "Profile Image" }
            }
     }
}
