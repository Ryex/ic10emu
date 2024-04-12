import { registerIconLibrary } from "@shoelace-style/shoelace/dist/utilities/icon-library.js";

registerIconLibrary("fa", {
  resolver: (name) => {
    const filename = name.replace(/^fa[rbs]-/, "");
    let folder = "regular";
    if (name.substring(0, 4) === "fas-") folder = "solid";
    if (name.substring(0, 4) === "fab-") folder = "brands";
    return `https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free@6.5.2/svgs/${folder}/${filename}.svg`;
  },
  mutator: (svg) => svg.setAttribute("fill", "currentColor"),
});
