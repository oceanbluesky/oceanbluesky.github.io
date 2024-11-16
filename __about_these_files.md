
1st:  npm run serve   
2nd:  http://localhost:8080


IF RUNNING VIA DOCKER:
1) open the Docker Desktop application to start the Docker daemon.
2) With -t githubio-app assigns the name githubio-app to the image and 
    specifies the current directory (where the Dockerfile is located) 
    as the build context:
        docker build --no-cache -t githubio-app .
3) Then with the following command start a container using this named image:
        docker run -p 8080:8080 githubio-app





Info:
https://dioxuslabs.com/learn/0.5/cookbook/tailwind/

run this to build tailwind.css and dx serve index.html:
 npm run serve   

Ensure 'cargo add manganis' has been run in terminal so manganis will be able
to collect and link the css within the index.html file compiled by Dioxus. (Otherwise it will be necessary to handcode this css link in the dist/assets/index.html file: <link rel="stylesheet" href="assets/styles/tailwind.css"> )




 _____


the 'npm commands' are in package.json (if files are moved update the package.json file as well)


* this needs 'npm install concurrently --save-dev' to ensure CSS watching and dx server run simultaneously 

* this alos needs 'cargo add manganis' to be run in the termiinal to ensure Dioxus can link the compoiled tailwind.css file from the index.html file Dioxus compiles
( src/styles/input.css is used to create the output css file dist/assets/styles/tailwind.css)

* 'tailwind.config.js' is used to tell tailwind where to find files to compile