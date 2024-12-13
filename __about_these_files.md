
# 1st:  
npm run serve   
# 2nd:  
http://localhost:8080


IF USING DOCKER:
1) open the Docker Desktop application to start the Docker daemon.
2) With -t githubio-app assigns the name githubio-app to the image and 
    specifies the current directory (where the Dockerfile is located) 
    as the build context:
        docker build --no-cache -t githubio-app .
3) Then with the following command start a container using this named image:
        docker run -p 8080:8080 githubio-app
4) Then open: http://localhost:8080 





Info:
https://dioxuslabs.com/learn/0.5/cookbook/tailwind/

run this to build tailwind.css and dx serve index.html:
 npm run serve   

Ensure 'cargo add manganis' has been run in terminal so manganis will be able
to collect and link the css within the index.html file compiled by Dioxus. (Otherwise it will be necessary to handcode this css link in the dist/assets/index.html file: <link rel="stylesheet" href="assets/styles/tailwind.css"> )




 _____


the 'npm commands' are in package.json (if files are moved update the package.json file as well)


* this needs 'npm install concurrently --save-dev' to ensure CSS watching and dx server run simultaneously 

* this also needs 'cargo add manganis' to be run in the termiinal to ensure Dioxus can link the compoiled tailwind.css file from the index.html file Dioxus compiles
( src/styles/input.css is used to create the output css file dist/assets/styles/tailwind.css)

* 'tailwind.config.js' is used to tell tailwind where to find files to compile



______

Dioxus Builds to Static Files:

When you build your Dioxus project (using trunk build or a similar command), it generates a dist directory containing: index.html, *.js (JavaScript files), *.wasm (WebAssembly files), CSS and other assets.

GitHub Pages Serves the Files
You simply push the contents of the dist directory to a branch like gh-pages (or main if configured) in your GitHub repository. GitHub Pages will host these files and serve them as a static website.

First build Your Dioxus App: 
Then run:
trunk build --release (er: or something similar)
This creates a dist directory with all the files needed to deploy.

Configure GitHub Pages: 
Ensure your repository has GitHub Pages enabled:
Go to your repository's Settings > Pages.
Select the branch (e.g., gh-pages) and specify / (root directory) for the source.

Push the Files...if you’re using a gh-pages branch, copy the contents of the dist directory into the branch and push:
git checkout --orphan gh-pages
git rm -rf .
cp -r dist/* .
git add .
git commit -m "Deploy Dioxus app"
git push origin gh-pages
Alternatively, use a deploy script like trunk serve with GitHub Pages.

Access Your Site:
Your site will be available at https://ericmachmer.github.io.