
# Currently best approach to running (without Docker)
        npm run build [to create the tailwinds.css before dioxus builds]
     then:
        dx serve --platform web [to serve the app locally]
        Serving at: http://127.0.0.1:8080     

        'npm run build' will create a tailwinds.css file in an assets/styles directory in the root -- this will then be copied over later to the public folder in target. Essenialy two identical tailwind.css files will be in the proejct -- this is ok. The assets directory located in the final output (e.g., public/assets/styles/tailwind.css) is the one referenced by the <link> tag in index.html. The root-level assets folder in your project is just a source directory that dx build copies into the final output directory.

        Detailed Explanation:

        Your project root may contain an assets folder with CSS, images, or other static files.
        When you run dx build, Dioxus copies these static assets into the final build output directory, often something like target/dx/<app_name>/release/web/public.
        The index.html generated or processed by Dioxus references these files using relative paths like assets/styles/tailwind.css. This path points to the assets folder inside the output directory (e.g., public/assets/styles/tailwind.css), not the original assets folder at your project root.


# REWORK THIS IF RETRYING WITH DOCKER
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

