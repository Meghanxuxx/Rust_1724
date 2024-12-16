# Final Report

**Team member**: Huilin Xu (1005969099, meghan.xu@mail.utoronto.ca), Shanzhi Zhang (, shanzhi.zhang@mail.utoronto.ca), Xian Wu (1009735146, amandaxian.wu@mail.utoronto.ca)

## Video Demo
URL: https://drive.google.com/file/d/1ctqJW5gmzOMTqyMHSDTDSnQ8PDx3Wjo2/view?usp=sharing

## Motivation

As international students and new immigrants, we recognize the challenges newcomers had in finding employment and the urgent need to address this gap. Currently, many new immigrants, students, and marginalized groups are facing some significant disadvantages in finding jobs, not only in Canada, but all around the world. We recently surveyed 200 people and discovered six major obstacles that are preventing them from reaching their employment aspirations:

1. **Language Barriers**: Many non-native speakers find it challenging to create resumes and cover letters that capture the subtleties of the local language and meet professional expectations.
2. **Professional Quality Standards**: Having high-quality, polished, and professional job application materials is essential for finding a job successfully, but it can sometimes feel out of reach for those who don’t have guidance or access to professional services.
3. **Limited Network Access**: Many immigrants find themselves without connections to local professionals in their field, which makes it challenging to access culturally relevant and industry-specific mentorship, results in professional services becoming unaffordable.
4. **Frequent Revisions Needed**: Job application materials, like resumes and cover letters, typically require several rounds of editing. However, mentors and advisors often can't provide the necessary extensive, ongoing support.
5. **Personalization challenges**: Creating application materials requires personal insight, making it hard for individuals to rely only on outside assistance to produce effective documents.
6. **Contextual Barriers**: Being unfamiliar with local industry standards, such as those in Canada, can impact one’s preparedness and potentially lower the success rates in job applications.

These barriers to job searching, caused by limited resources and a lack of knowledge about the position or company, further affect the two most important documents in the application process—the resume and the cover letter. Our project chose to focus on cover letters, partly due to the short length of the program, but more importantly because applicants often overlook the significance of cover letters or struggle to meet expectations. While cover letters are optional in the job application process for some companies, there are still a significant number of positions that require applicants to submit them as an opportunity to demonstrate their understanding of the position and the organization, career experience, and ambition to help the applicant stand out from the pool of candidates. Writing an effective cover letter can be challenging and requires:

1. Overview of job role and responsibilities
2. Alignment with the company's culture and mission
3. Relevant professional experience matching the job description
4. Strong language proficiency
5. Genuine enthusiasm for the role
6. Demonstrated future potential

Therefore combining our own needs as well as those of our target group, the goal of our project is to develop an AI-powered platform designed to increase the competitiveness of users in their job search by producing customized cover letters that meet linguistic and professional standards. Our tool will provide native-level language assistance and industry-appropriate expressions that are customized to each user's unique experience and needs. And with this solution, we hope to reduce the barriers faced by newcomers and marginalized groups, increase their employability, promote economic integration, and contribute to a more inclusive workforce. This AI-driven approach not only meets an unfilled need, but also puts us at the forefront of the market as we scale to support an increasingly globalized and diverse talent pool.

## Objective

The aim of this project is to develop a website integrated with Large Language Models (LLMs) in Rust to assist users in creating professional and customized cover letters to improve their job search prospects. It involves optimizing prompts to improve tool performance, as well as creating an interactive, user-friendly web interface using the framework. The project has several key milestones, which are explained in detail below from both the front-end and back-end perspectives.

### 1. Front-End Development

Given the project's features, we decided to use a website as our platform, and for a website to be effective, it's important to think about how users will navigate and interact with it. We needed to create a detailed UI/UX design document that would make sure the interface is easy to understand, accessible, and user-friendly. This document will describe how users will engage with the website and include all potential features, serving as a clear guide for development. On top of this, the front-end also needs to to include all the features and connect with the back-end API to allow for real-time updates and data syncing, ensuring a smooth content generation experience for the user. Our main goals can be summarized as follows:

1. **Improve user experience**: By designing an easy-to-use interface and providing clear guidance, users can quickly create a customized cover letter with just a few clicks and simple steps, making it easier for them to learn how to use the tool.
2. **Real-time feedback**: The front-end shows instant suggestions and reminders as users fill in their personal information, such as highlighting important skills, pointing out missing information, and providing interactive feedback to help adjust the length and tone of the writing. (Will be discussed later in feature section)
3. **Flexible component design**: The front-end framework is built in a way that allows for reusable and expandable components, so new features can be added easily in the future while keeping the code organized and easy to maintain.

![Our Front-End Design](https://github.com/Meghanxuxx/Rust_1724/blob/report/Frontend/static/UI.jpg?raw=true)

### 2. Back-End Development

Since the back-end needs to provide reliable support for the front-end's real-time interactions and data processing, our design aims to create clear and effective API interfaces that can quickly respond to user requests, ensures that the front-end receives accurate feedback. Meanwhile, since generating high-quality customized cover letters relies heavily on the prompt design, we will fine-tune and optimize the prompts to maximize the output quality, so that we can generate customized cover letter content that better matches the job requirements and personal background of the users. Finally, in order for users to view their past communications, the backend needs to satisfy storage functionality, including user account information as well as chat logs. So the objectives of our backend can be summarized as:

1. **Provide API Services**: Use clear endpoints (e.g., /api/step1, /api/step2, /api/step3) to receive multi-step input from the user, and after the user has completed submitting all the necessary information, send the integrated data to the LLM API for customized cover letter content and return it to the front-end.
2. **Optimize Prompt**: Perform basic processing and splicing of user inputs in the back-end, pass them to the LLM API as prompt words, and adjust the parameters according to the configuration file so that the model generates output content that better matches the job description and user background.
3. **Data Storage and Access Control**: Store the user's basic information and chat history in a local JSON file. Users can later revisit the data submitted and content generated in the past.

### 3. Project Objective Milestones Conclusion

The overall development of the project can be divided into the following key milestones:

1. **UI/UX Design**: Create and iteratively improve the design based on user feedback.
2. **Rust Front-End Framework**: Develop the front-end using a Rust framework.
3. **Front-End and Back-End API Interface**: Build a real-time interface for communication between the front-end and back-end.
4. **LLM API Integration**: Successfully incorporate the LLM API with optimized prompts.
5. **Prompt Engineering and Optimization**: Design and refine prompts for high-quality outputs.
6. **Performance Optimization and Testing**: Test the system extensively and make necessary optimizations to ensure optimal performance.

---
# Features

The main features are organized into three key aspects:

### Login System

- Allows users to sign up for a new account.
- Enables users to log in to an existing account.
- Displays user information on the main page.

### Real-Time Communication System

- Provides hint questions to help users input their information.
- Generates a complete cover letter as output on the summary page.
- Supports starting a new conversation.

### History Management System

- Displays a history of the latest corresponding output.

---
## User Guide

### Step 1: Create an Account

- Sign up for a new account and log in to access the system from the home page.
<img width="1412" alt="image" src="https://github.com/user-attachments/assets/7e9e3ad5-0dd1-4ad1-9374-13abdd059925" />

<img width="1407" alt="image" src="https://github.com/user-attachments/assets/8eb15343-d3ac-4552-b32b-c984dfc37d3b" />

### Step 2: Input Information

- Follow the instructions in the left-hand column.
- Provide answers to the hint questions in the right-hand section of Step 1.

<img width="1406" alt="image" src="https://github.com/user-attachments/assets/d2ffb292-c652-4206-b61a-8674e982f626" />

### Step 3: Complete Additional Steps

- Repeat the process for Step 2 and Step 3 to provide additional information.

### Step 4: Generate and Review

- View the fully generated cover letter on the summary page.
<img width="1406" alt="image" src="https://github.com/user-attachments/assets/3b0c238b-05c8-49b5-ba78-297370196d49" />


We also provide a test case for users to experience the system functionality.


### Test case

**Step 1**: I am Leo. I graduated from the University of Toronto with a master degree in the Electrical and Computer Engineering. I am proficient in Python, Rust, C++, and familiar with JavaScript, TypeScript and Verilog. I won the hackathon competition last year with a project about the to-do list finished by JavaScript and Python.I interned at Intel for four months as a software engineer intern working in the project about the deployment of cloud during my senior year. I interned at Yelp for six months and finished a project about building a website and doing data analytics to analyze customers habits there during my master’s first year.

**Step 2**: I want to apply for a new grad entry level software engineer position starting from 2025 Jun. Here is the job description. 

**Minimum qualifications:**

- Bachelor’s degree or equivalent practical experience.
- 2 years of experience with software development in one or more programming languages, or 1 year of experience with an advanced degree.
- 2 years of experience with data structures or algorithms.
- Experience working with SQL, Data Analysis.

**Preferred qualifications:**

- Master's degree or PhD in Computer Science or related technical fields.
- Experience developing accessible technologies.
- Experience in Search Quality, Large Language Model, Personalization.
- Experience coding in C++ or Python.

**About The Job**

Google's software engineers develop the next-generation technologies that change how billions of users connect, explore, and interact with information and one another. Our products need to handle information at massive scale, and extend well beyond web search. We're looking for engineers who bring fresh ideas from all areas, including information retrieval, distributed computing, large-scale system design, networking and data storage, security, artificial intelligence, natural language processing, UI design and mobile; the list goes on and is growing every day. As a software engineer, you will work on a specific project critical to Google’s needs with opportunities to switch teams and projects as you and our fast-paced business grow and evolve. We need our engineers to be versatile, display leadership qualities and be enthusiastic to take on new problems across the full-stack as we continue to push technology forward.

In this role, you will manage project priorities, deadlines, and deliverables. You will design, develop, test, deploy, maintain, and enhance software solutions.

People shop on Google more than a billion times a day - and the Commerce team is responsible for building the experiences that serve these users. The mission for Google Commerce is to be an essential part of the shopping journey for consumers - from inspiration to to a simple and secure checkout experience - and the best place for retailers/merchants to connect with consumers. We support and partner with the commerce ecosystem, from large retailers to small local merchants, to give them the tools, technology and scale to thrive in today’s digital world.

**Step 3**: Please write it in a formal and confident tone around 300 words. I hope that it could exhibit my professional skills well.

---








## Contribution

UI/UX design: Huilin Xu

Frontend: Huilin Xu

Backend (API integration, infrastructure): Shanzhi Zhang

Backend (Log-in system): Huilin Xu (Meghan)

Backend (History): Xian Wu

Prompt Engineering & Document & Test: Xian Wu

---

## Conclusion

Through this project, we practiced our Rust skills by developing front-end pages using the Yew framework and implementing backend functions to support API request/response communication, a login system, and various other supportive features.

During the development process, we encountered several challenges due to the unique characteristics of the Rust programming language. Additionally, the limited availability of up-to-date guidance and documentation posed an obstacle to our progress. However, by leveraging the detailed error messages and hints provided by Rust, we were able to resolve most of these issues and successfully achieve a demo-level implementation of the product.

For the backend development, we gained a clearer understanding of the different approaches required when working with Python and Rust. In the initial stages of building a new feature, it’s often difficult to fully account for all the details, such as data types, API structures, and other specifics. Compared to Python’s agile and flexible development style, Rust enforces a much stricter design philosophy. Its compiler performs rigorous checks for types, lifetimes, and other elements, treating any violations as errors. While this ensures the safety and robustness of the system, it also makes initializing projects or features more cumbersome. We found ourselves spending significant time fixing related functions across multiple files just to modify one or two interfaces during the process.
